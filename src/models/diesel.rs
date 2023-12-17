use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};

#[derive(Queryable, Identifiable, Clone)]
pub struct Channel {
    pub id: i32,
    pub alias_id: i32,
    pub alias_name: String,
    pub joined_at: NaiveDateTime,
    pub opt_outed_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel {
    pub alias_id: i32,
    pub alias_name: String,
}

#[derive(Queryable, Identifiable, Associations, Clone)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(table_name = channel_preferences)]
pub struct ChannelPreference {
    pub id: i32,
    pub channel_id: i32,
    pub prefix: String,
    pub language: String,
}

#[derive(Insertable)]
#[diesel(table_name = channel_preferences)]
pub struct NewChannelPreference {
    pub channel_id: i32,
    pub prefix: String,
    pub language: String,
}

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub alias_id: i32,
    pub alias_name: String,
    pub joined_at: NaiveDateTime,
    pub opt_outed_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub alias_id: i32,
    pub alias_name: String,
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::ActionStatuses"]
pub enum ActionStatus {
    Ok,
    Error,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = actions)]
pub struct Action {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub command_name: String,
    pub arguments: Option<String>,
    pub response: String,
    pub status: ActionStatus,
    pub sent_at: NaiveDateTime,
    pub processed_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = actions)]
pub struct NewAction {
    pub channel_id: i32,
    pub user_id: i32,
    pub command_name: String,
    pub arguments: Option<String>,
    pub response: String,
    pub status: ActionStatus,
    pub sent_at: NaiveDateTime,
    pub processed_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Clone)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(table_name = timers)]
pub struct Timer {
    pub id: i32,
    pub name: String,
    pub channel_id: i32,
    pub messages: Vec<String>,
    pub interval_sec: i64,
    pub last_executed_at: NaiveDateTime,
    pub is_enabled: bool,
}

#[derive(Insertable)]
#[diesel(table_name = timers)]
pub struct NewTimer {
    pub name: String,
    pub channel_id: i32,
    pub messages: Vec<String>,
    pub interval_sec: i64,
}

#[derive(Queryable, Identifiable, Associations, Clone)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(table_name = custom_commands)]
pub struct CustomCommand {
    pub id: i32,
    pub channel_id: i32,
    pub name: String,
    pub messages: Vec<String>,
    pub is_enabled: bool,
    pub created_at: NaiveDateTime,
    pub last_executed_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = custom_commands)]
pub struct NewCustomCommand {
    pub channel_id: i32,
    pub name: String,
    pub messages: Vec<String>,
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::EventType"]
pub enum EventType {
    Live,
    Offline,
    Title,
    Category,
    Custom,
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::EventFlag"]
pub enum EventFlag {
    Massping,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
pub struct Event {
    pub id: i32,
    pub channel_id: i32,
    pub target_alias_id: Option<i32>,
    pub custom_alias_id: Option<String>,
    pub event_type: EventType,
    pub flags: Vec<EventFlag>,
    pub message: String,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub channel_id: i32,
    pub target_alias_id: Option<i32>,
    pub custom_alias_id: Option<String>,
    pub event_type: EventType,
    pub message: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Event, foreign_key = event_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct EventSubscription {
    pub id: i32,
    pub event_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = event_subscriptions)]
pub struct NewEventSubscription {
    pub event_id: i32,
    pub user_id: i32,
}
