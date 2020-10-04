use std::collections::HashMap;


pub struct AppState {
    pub(crate) app_name: String,
    pub(crate) tokens: HashMap<String, String>
}