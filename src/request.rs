use std::default::Default;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum ParseMode {
    Text,
    Markdown,
    Html,
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
    #[serde(skip_serializing_if = "ParseMode::is_text")]
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

#[derive(Clone, Debug, Serialize, Default)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    pub cache_time: Option<i64>,
    pub is_personal: Option<bool>,
    pub next_offset: String,
    pub switch_pm_text: Option<String>,
    pub switch_pm_parameter: Option<String>,
}
impl AnswerInlineQuery {
    pub fn new(
        inline_query_id: String,
        results: Vec<InlineQueryResult>,
        next_offset: String,
    ) -> AnswerInlineQuery {
        AnswerInlineQuery {
            inline_query_id,
            results,
            next_offset,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum InlineQueryResult {
    Article {
        id: String,
        title: String,
        message_text: String,
        //TODO
    },
}
impl InlineQueryResult {
    pub fn article(id: String, title: String, message_text: String) -> InlineQueryResult {
        InlineQueryResult::Article {
            id,
            title,
            message_text,
        }
    }
}
