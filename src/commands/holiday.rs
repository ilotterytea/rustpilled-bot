use async_trait::async_trait;
use chrono::{Datelike, Utc};
use rand::Rng;
use reqwest::Client;
use twitch_irc::message::PrivmsgMessage;

use crate::{
    command::Command,
    instance_bundle::InstanceBundle,
    localization::LineId,
    message::ParsedPrivmsgMessage,
    models::diesel::{Channel, ChannelPreference, User},
    shared_variables::HOLIDAY_V1_API_URL,
};

pub struct HolidayCommand;

#[async_trait]
impl Command for HolidayCommand {
    fn get_name(&self) -> String {
        "holiday".to_string()
    }

    async fn execute(
        &self,
        instance_bundle: &InstanceBundle,
        data_message: PrivmsgMessage,
        message: ParsedPrivmsgMessage,
        _channel: &Channel,
        channel_preferences: &ChannelPreference,
        _user: &User,
    ) -> Option<Vec<String>> {
        let split_msg = message.message.unwrap().split('.').collect::<Vec<&str>>();

        let date = Utc::now();

        let day = if let Some(day) = split_msg.get(0) {
            if let Ok(day) = day.parse::<u32>() {
                day
            } else {
                date.day()
            }
        } else {
            date.day()
        };

        let month = if let Some(month) = split_msg.get(1) {
            if let Ok(month) = month.parse::<u32>() {
                month
            } else {
                date.month()
            }
        } else {
            date.month()
        };

        let client = Client::default();

        let holidays = if let Ok(response) = client
            .get(format!("{}/{}/{}", HOLIDAY_V1_API_URL, month, day))
            .send()
            .await
        {
            if let Ok(parsed_json) = response.json::<Vec<String>>().await {
                parsed_json
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        if holidays.is_empty() {
            return Some(vec![instance_bundle
                .localizator
                .get_formatted_text(
                    channel_preferences.language.clone().unwrap().as_str(),
                    LineId::CommandHolidayEmpty,
                    vec![
                        data_message.sender.name.clone(),
                        day.to_string(),
                        month.to_string(),
                    ],
                )
                .unwrap()]);
        }

        let mut rand = rand::thread_rng();
        let index = rand.gen_range(0..holidays.len());
        let holiday = holidays.get(index).unwrap();

        Some(vec![instance_bundle
            .localizator
            .get_formatted_text(
                channel_preferences.language.clone().unwrap().as_str(),
                LineId::CommandHolidayResponse,
                vec![data_message.sender.name.clone(), holiday.into()],
            )
            .unwrap()])
    }
}
