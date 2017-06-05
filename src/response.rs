#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub date: i32,
    pub chat: Chat,
    pub text: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename="type")]
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
}
#[derive(Clone, Copy,  PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum ChatType {
    Private,
    Group,
    SuperGroup,
    Channel,
}
