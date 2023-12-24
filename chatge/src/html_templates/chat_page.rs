use askama::Template;

#[derive(Template)]
#[template(path = "chat_page.html")]
pub struct ChatPage {}

#[derive(Template)]
#[template(path = "chat_message.html")]
pub struct Message {
    pub from: String,
    pub time: String,
    pub text: String,
}
