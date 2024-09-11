#[derive(serde::Serialize)]
pub struct Routes {
    // template handlers
    pub static_files: &'static str,
    pub main_page: &'static str,
    pub knowledge_base: &'static str,
    pub contacts: &'static str,
    // api
    pub create_description: &'static str,
    pub delete_description: &'static str,
}

impl Default for Routes {
    fn default() -> Self {
        Routes {
            static_files: "/static",
            main_page: "/",
            knowledge_base: "/knowledge_base",
            contacts: "/contacts",
            create_description: "/descriptions",
            delete_description: "/descriptions/:id",
        }
    }
}