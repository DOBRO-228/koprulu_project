#[derive(serde::Serialize)]
pub struct Routes {
    pub main_page: &'static str,
    pub knowledge_base: &'static str,
    pub contacts: &'static str,
}

impl Default for Routes {
    fn default() -> Self {
        Routes {
            main_page: "/",
            knowledge_base: "/knowledge_base",
            contacts: "/contacts",
        }
    }
}