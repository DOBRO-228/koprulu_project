use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use common::app_state::AppState;
use tera::Context;

pub async fn render_main_page(app_state: State<AppState>) -> impl IntoResponse {
    // Render the template using the Tera instance from AppState
    let context = Context::new();
    match app_state.templates.render("main_page.html", &context) {
        Ok(rendered_html) => Html(rendered_html).into_response(), // Return the rendered HTML as response
        Err(err) => {
            eprintln!("Template error: {}", err); // Log the error for debugging
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response() // Return error response
        }
    }
}
