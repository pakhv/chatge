use askama::Template;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
};

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        handle_template_render(self)
    }
}

impl<T> Into<Response<Body>> for HtmlTemplate<T>
where
    T: Template,
{
    fn into(self) -> Response<Body> {
        handle_template_render(self)
    }
}

fn handle_template_render<T>(template: HtmlTemplate<T>) -> Response<Body>
where
    T: Template,
{
    match template.0.render() {
        Ok(result) => Html(result).into_response(),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Failed to render template. {err}")))
            .unwrap(),
    }
}
