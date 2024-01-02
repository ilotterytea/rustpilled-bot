use std::{collections::HashSet, str::FromStr};

use async_trait::async_trait;
use diesel::{delete, insert_into, BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl};
use eyre::Result;
use twitch_api::{
    helix::chat::GetChattersRequest,
    types::{NicknameRef, UserId},
};

use crate::{
    commands::{
        request::Request,
        response::{Response, ResponseError},
        Command, CommandArgument,
    },
    instance_bundle::InstanceBundle,
    localization::LineId,
    utils::{diesel::establish_connection, split_and_wrap_lines},
};

use common::{
    models::{Event, EventFlag, EventSubscription, EventType, LevelOfRights, NewEvent, User},
    schema::{events::dsl as ev, users::dsl as us},
};

pub struct EventCommand;

#[async_trait]
impl Command for EventCommand {
    fn get_name(&self) -> String {
        "event".to_string()
    }

    fn required_rights(&self) -> LevelOfRights {
        LevelOfRights::Moderator
    }

    fn get_subcommands(&self) -> Vec<String> {
        vec!["on".to_string(), "off".to_string(), "call".to_string()]
    }

    async fn execute(
        &self,
        instance_bundle: &InstanceBundle,
        request: Request,
    ) -> Result<Response, ResponseError> {
        let subcommand_id = match request.subcommand_id {
            Some(v) => v,
            None => {
                return Err(ResponseError::NotEnoughArguments(
                    CommandArgument::Subcommand,
                ))
            }
        };

        if request.message.is_none() {
            return Err(ResponseError::NotEnoughArguments(CommandArgument::Target));
        }

        let message = request.message.unwrap();
        let mut message_split = message.split(" ").collect::<Vec<&str>>();

        let (target_name, event_type) = match message_split.first() {
            Some(v) => {
                let v = v.to_string();

                message_split.remove(0);

                let vec = v.split(":").collect::<Vec<&str>>();

                match (vec.first(), vec.get(1)) {
                    (Some(target_name), Some(event_type))
                        if !target_name.is_empty() && !event_type.is_empty() =>
                    {
                        (
                            target_name.to_string(),
                            EventType::from_str(event_type).unwrap(),
                        )
                    }
                    _ => return Err(ResponseError::IncorrectArgument(v)),
                }
            }
            None => return Err(ResponseError::NotEnoughArguments(CommandArgument::Target)),
        };

        let target_id = match instance_bundle
            .twitch_api_client
            .get_user_from_login(
                NicknameRef::from_str(target_name.as_str()),
                &*instance_bundle.twitch_api_token,
            )
            .await
        {
            Ok(Some(v)) => v.id.take().parse::<i32>().unwrap(),
            _ => -1,
        };

        let name_and_type = format!("{}:{}", target_name, event_type.to_string());

        if target_id == -1 && event_type != EventType::Custom {
            return Err(ResponseError::NotFound(target_name));
        }

        let conn = &mut establish_connection();
        let events = Event::belonging_to(&request.channel)
            .filter(ev::event_type.eq(&event_type))
            .load::<Event>(conn)
            .expect("Failed to load events");

        let event = events.iter().find(|x| {
            if x.event_type == EventType::Custom {
                x.custom_alias_id.clone().unwrap().eq(&target_name)
            } else {
                x.target_alias_id.clone().unwrap().eq(&target_id)
            }
        });

        let response = match (subcommand_id.as_str(), event) {
            ("call", Some(e)) => {
                let subs = EventSubscription::belonging_to(&e)
                    .load::<EventSubscription>(conn)
                    .expect("Failed to load subscriptions for event");

                let users = us::users
                    .load::<User>(conn)
                    .expect("Failed to load users for event");

                let users = users
                    .iter()
                    .filter(|x| subs.iter().any(|y| y.user_id == x.id))
                    .map(|x| format!("@{}", x.alias_name))
                    .collect::<Vec<String>>();

                let mut subs: HashSet<String> = HashSet::new();

                subs.extend(users);

                if e.flags.contains(&EventFlag::Massping) {
                    let broadcaster_id = request.channel.alias_id.to_string();
                    let moderator_id = instance_bundle.twitch_api_token.user_id.clone().take();

                    if let Ok(chatters) = instance_bundle
                        .twitch_api_client
                        .req_get(
                            GetChattersRequest::new(broadcaster_id.as_str(), moderator_id.as_str()),
                            &*instance_bundle.twitch_api_token,
                        )
                        .await
                    {
                        let chatters = chatters
                            .data
                            .iter()
                            .map(|x| format!("@{}", x.user_login))
                            .collect::<HashSet<String>>();

                        subs.extend(chatters);
                    } else {
                        return Err(ResponseError::InsufficientRights);
                    }
                }

                if subs.is_empty() {
                    return Ok(Response::Single(format!("⚡ {}", e.message)));
                }

                let formatted_subs = split_and_wrap_lines(
                    subs.into_iter()
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str(),
                    ", ",
                    300 - e.message.len(),
                );

                let formatted_subs = formatted_subs
                    .iter()
                    .map(|x| format!("⚡ {} · {}", e.message, x))
                    .collect::<Vec<String>>();

                Response::Multiple(formatted_subs)
            }

            ("on", Some(_)) => return Err(ResponseError::NamesakeCreation(name_and_type)),
            ("on", None) => {
                let message = message_split.join(" ");

                if message.is_empty() {
                    return Err(ResponseError::NotEnoughArguments(CommandArgument::Message));
                }

                insert_into(ev::events)
                    .values([NewEvent {
                        channel_id: request.channel.id,
                        target_alias_id: if event_type != EventType::Custom {
                            Some(target_id)
                        } else {
                            None
                        },
                        custom_alias_id: if event_type == EventType::Custom {
                            Some(target_name.clone())
                        } else {
                            None
                        },
                        event_type: event_type.clone(),
                        message,
                    }])
                    .execute(conn)
                    .expect("Failed to create a new event");

                if event_type != EventType::Custom && target_id != -1 {
                    let mut ids = instance_bundle
                        .twitch_livestream_websocket_data
                        .lock()
                        .await;

                    ids.awaiting_channel_ids
                        .push(UserId::new(target_id.to_string()));

                    drop(ids);
                }

                Response::Single(
                    instance_bundle
                        .localizator
                        .get_formatted_text(
                            request.channel_preference.language.as_str(),
                            LineId::EventOn,
                            vec![
                                request.sender.alias_name.clone(),
                                target_name,
                                event_type.to_string(),
                            ],
                        )
                        .unwrap(),
                )
            }

            ("off", Some(e)) => {
                delete(ev::events.find(e.id))
                    .execute(conn)
                    .expect("Failed to delete the event");

                // TODO: Delete a subscription from the websocket if it is the last subscriber

                Response::Single(
                    instance_bundle
                        .localizator
                        .get_formatted_text(
                            request.channel_preference.language.as_str(),
                            LineId::EventOff,
                            vec![
                                request.sender.alias_name.clone(),
                                target_name,
                                event_type.to_string(),
                            ],
                        )
                        .unwrap(),
                )
            }
            ("off", None) => return Err(ResponseError::NotFound(name_and_type)),
            _ => {
                return Err(ResponseError::SomethingWentWrong);
            }
        };

        Ok(response)
    }
}
