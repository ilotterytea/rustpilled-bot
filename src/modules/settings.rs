use async_trait::async_trait;
use diesel::{update, ExpressionMethods, RunQueryDsl};
use eyre::Result;

use crate::{
    commands::{
        request::Request,
        response::{Response, ResponseError},
        Command,
    },
    instance_bundle::InstanceBundle,
    localization::LineId,
    schema::channel_preferences::dsl as chp,
    utils::diesel::establish_connection,
};

pub struct SettingsCommand;

#[async_trait]
impl Command for SettingsCommand {
    fn get_name(&self) -> String {
        "set".to_string()
    }

    fn get_subcommands(&self) -> Vec<String> {
        vec!["locale".to_string(), "prefix".to_string()]
    }

    async fn execute(
        &self,
        instance_bundle: &InstanceBundle,
        mut request: Request,
    ) -> Result<Response, ResponseError> {
        let subcommand_id = match request.subcommand_id {
            Some(v) => v,
            None => return Err(ResponseError::NoSubcommand),
        };

        if request.message.is_none() {
            return Err(ResponseError::NoMessage);
        }

        let message = request.message.unwrap();

        let conn = &mut establish_connection();

        let response = match subcommand_id.as_str() {
            "locale" => {
                let locales = instance_bundle.localizator.localization_names();

                if !locales.contains(&&message) {
                    return Err(ResponseError::WrongArguments);
                }

                request.channel_preference.language = message.clone();

                update(chp::channel_preferences)
                    .set(chp::language.eq(message))
                    .execute(conn)
                    .expect("Failed to update the channel preference");

                instance_bundle
                    .localizator
                    .get_formatted_text(
                        request.channel_preference.language.as_str(),
                        LineId::SettingsLocale,
                        vec![request.sender.alias_name],
                    )
                    .unwrap()
            }
            "prefix" => {
                update(chp::channel_preferences)
                    .set(chp::prefix.eq(message.clone()))
                    .execute(conn)
                    .expect("Failed to update the channel preference");

                instance_bundle
                    .localizator
                    .get_formatted_text(
                        request.channel_preference.language.as_str(),
                        LineId::SettingsPrefix,
                        vec![request.sender.alias_name, message],
                    )
                    .unwrap()
            }
            _ => return Err(ResponseError::WrongArguments),
        };

        Ok(Response::Single(response))
    }
}