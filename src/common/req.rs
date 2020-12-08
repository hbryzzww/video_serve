use serde::Deserialize;

#[derive(Deserialize)]
pub struct Search {
    pub keyword: String,
}
