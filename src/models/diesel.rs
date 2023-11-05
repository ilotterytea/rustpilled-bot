use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};

#[derive(Queryable, Identifiable)]
pub struct Channel {
    pub id: i32,
    pub alias_id: i32,
    pub joined_at: NaiveDateTime,
    pub opt_outed_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel {
    pub alias_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(table_name = channel_preferences)]
pub struct ChannelPreference {
    pub id: i32,
    pub channel_id: i32,
    pub prefix: Option<String>,
    pub language: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = channel_preferences)]
pub struct NewChannelPreference {
    pub channel_id: i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub alias_id: i32,
    pub joined_at: NaiveDateTime,
    pub opt_outed_at: Option<NaiveDateTime>,
    pub is_super_user: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub alias_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = actions)]
pub struct Action {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub command: String,
    pub attributes: Option<String>,
    pub full_message: String,
    pub response: Option<String>,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = actions)]
pub struct NewAction {
    pub channel_id: i32,
    pub user_id: i32,
    pub command: String,
    pub attributes: Option<String>,
    pub full_message: String,
    pub response: Option<String>,
}
