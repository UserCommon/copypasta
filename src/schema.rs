use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsOptions {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePastaSchema {
    pub lang: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePastaSchema {
    pub lang: String,
    pub text: String,
}
