use std::env;

use async_trait::async_trait;
use eyre::Result;
use log::error;

use crate::{
    commands::{
        request::Request,
        response::{Response, ResponseError},
        Command, CommandArgument,
    },
    instance_bundle::InstanceBundle,
    localization::LineId,
    models::stats::{ChannelEmote, ChannelEmoteUsage, Response as StatsResponse},
};

pub struct EmoteCountCommand;

#[async_trait]
impl Command for EmoteCountCommand {
    fn get_name(&self) -> String {
        "ecount".to_string()
    }

    async fn execute(
        &self,
        instance_bundle: &InstanceBundle,
        request: Request,
    ) -> Result<Response, ResponseError> {
        if env::var("STATS_API_HOSTNAME").is_err() {
            error!("Tried to run the !ecount command, but STATS_API_HOSTNAME is not set in the .env file");
            return Err(ResponseError::SomethingWentWrong);
        }

        if let Some(message) = request.message.clone() {
            let channel_id = request.channel.alias_id.to_string();
            if let Some(response) = self.fetch_channel_emotes(channel_id.clone()).await {
                if response.status_code != 200 {
                    return Err(ResponseError::ExternalAPIError(
                        response.status_code,
                        response.message,
                    ));
                }

                let emotes = response.data.unwrap_or_default();

                if let Some(response) = self.fetch_channel_emote_usages(channel_id).await {
                    if response.status_code != 200 {
                        return Err(ResponseError::ExternalAPIError(
                            response.status_code,
                            response.message,
                        ));
                    }

                    let emote_usages = response.data.unwrap_or_default();

                    if let Some(emote) = emotes.iter().find(|x| x.name.eq(&message)) {
                        let usage =
                            match emote_usages.iter().find(|x| x.emote_id.eq(&emote.emote_id)) {
                                Some(usage) => usage.usage_count,
                                None => 0,
                            };

                        return Ok(Response::Single(
                            instance_bundle.localizator.formatted_text_by_request(
                                &request,
                                LineId::EmoteCountUsage,
                                vec![
                                    instance_bundle
                                        .localizator
                                        .get_literal_text(
                                            request.channel_preference.language.as_str(),
                                            LineId::Provider7TV,
                                        )
                                        .unwrap(),
                                    emote.name.clone(),
                                    usage.to_string(),
                                ],
                            ),
                        ));
                    }

                    return Ok(Response::Single(
                        instance_bundle.localizator.formatted_text_by_request(
                            &request,
                            LineId::EmoteCountNotFound,
                            vec![
                                instance_bundle
                                    .localizator
                                    .get_literal_text(
                                        request.channel_preference.language.as_str(),
                                        LineId::Provider7TV,
                                    )
                                    .unwrap(),
                                message,
                            ],
                        ),
                    ));
                }
            }
        }

        Err(ResponseError::NotEnoughArguments(CommandArgument::Target))
    }
}

impl EmoteCountCommand {
    async fn fetch_channel_emotes(
        &self,
        channel_id: String,
    ) -> Option<StatsResponse<Vec<ChannelEmote>>> {
        let url = format!(
            "{}/api/v1/channel/twitch/{}/emotes",
            env::var("STATS_API_HOSTNAME")
                .expect("STATS_API_HOSTNAME must be set for !ecount command"),
            channel_id
        );

        let client = reqwest::Client::default();
        let mut request = client.get(url);

        if let Ok(credentials) = env::var("STATS_API_PASSWORD") {
            let mut split = credentials.split(':').collect::<Vec<&str>>();

            if !split.is_empty() {
                let name = split[0];
                split.remove(0);

                let password = split.join(":");

                request = request.basic_auth(
                    name,
                    if password.is_empty() {
                        None
                    } else {
                        Some(password)
                    },
                );
            }
        }

        if let Ok(response) = request.send().await {
            if let Ok(data) = response.json::<StatsResponse<Vec<ChannelEmote>>>().await {
                return Some(data);
            }
        }

        None
    }
    async fn fetch_channel_emote_usages(
        &self,
        channel_id: String,
    ) -> Option<StatsResponse<Vec<ChannelEmoteUsage>>> {
        let url = format!(
            "{}/api/v1/channel/twitch/{}/emotes/usage",
            env::var("STATS_API_HOSTNAME")
                .expect("STATS_API_HOSTNAME must be set for !ecount command"),
            channel_id
        );

        let client = reqwest::Client::default();
        let mut request = client.get(url);

        if let Ok(credentials) = env::var("STATS_API_PASSWORD") {
            let mut split = credentials.split(':').collect::<Vec<&str>>();

            if !split.is_empty() {
                let name = split[0];
                split.remove(0);

                let password = split.join(":");

                request = request.basic_auth(
                    name,
                    if password.is_empty() {
                        None
                    } else {
                        Some(password)
                    },
                );
            }
        }

        if let Ok(response) = request.send().await {
            if let Ok(data) = response
                .json::<StatsResponse<Vec<ChannelEmoteUsage>>>()
                .await
            {
                return Some(data);
            }
        }

        None
    }
}
