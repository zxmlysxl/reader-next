use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSubscription {
    pub id: Option<i64>,
    #[serde(skip)]
    pub user_namespace: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}
