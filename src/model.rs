use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PastaModel {
    pub id: uuid::Uuid,
    pub lang: Option<String>,
    pub text: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// In tutorial man using mysql where boolean type not exists
/// So this struct used as representation of PastaModel
/// SOOO we can just not use this thing i guess
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PastaResponse {
    pub id: i32,
    pub lang: Option<String>,
    pub text: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
