#[derive(serde::Serialize)]
pub struct Routes {
    // Template handlers
    pub static_files: &'static str,
    pub main_page: &'static str,
    pub knowledge_base: &'static str,
    pub contacts: &'static str,
    // API
    pub get_all_descriptions: &'static str,
    pub get_description: &'static str,
    pub create_description: &'static str,
    pub update_description: &'static str,
    pub delete_description: &'static str,
    pub delete_all_descriptions: &'static str,
}

impl Default for Routes {
    fn default() -> Self {
        Routes {
            static_files: "/static",
            // Templates
            main_page: "/",
            knowledge_base: "/knowledge_base",
            contacts: "/contacts",
            // API
            get_all_descriptions: "/descriptions",
            get_description: "/descriptions/:id",
            create_description: "/descriptions",
            update_description: "/descriptions/:id",
            delete_description: "/descriptions/:id",
            delete_all_descriptions: "/descriptions",
        }
    }
}
