use askama::Template;

#[derive(Template)]
#[template(path = "chat_page.html")]
pub struct ChatPage {}

#[derive(Template)]
#[template(path = "clicked.html")]
pub struct Clicked {
    pub message: String,
}
