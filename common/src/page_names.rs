use serde::Serialize;

#[derive(Serialize)]
pub enum PageNames {
    MainPage,
    Contacts,
}