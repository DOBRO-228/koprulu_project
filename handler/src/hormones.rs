// use axum::{
//     extract,
//     response::IntoResponse,
// };
// use crate::utils::HtmlTemplate;
//
// #[template(path = "hormones.html")]
// struct HormonesListTemplate {
//     hormones: Vec<String>,
// }
//
// async fn list_hormones(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
//     hormones = ...
//     let template = HormonesListTemplate { hormones };
//     HtmlTemplate(template)
// }
