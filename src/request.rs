use std::default::Default;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum ParseMode {
    Text,
    Markdown,
    Html
}
impl ParseMode {
    fn is_text(&self) -> bool {
        *self == ParseMode::Text
    }
}
impl Default for ParseMode {
    fn default() -> ParseMode {
        ParseMode::Text
    }
}
#[derive(Clone, Debug, Serialize, Default)]
pub struct Message {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if="ParseMode::is_text")]
    pub parse_mode: ParseMode,
    //TODO
}
impl Message {
    pub fn new(chat_id: i64, text: String) -> Message {
        Message {
            chat_id,
            text,
            ..Default::default()
        }
    }
    pub fn parse_mode(mut self, mode: ParseMode) -> Message {
        self.parse_mode = mode;
        self
    }
}
