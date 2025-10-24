use serde::{Serialize, Deserialize, Deserializer};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde_with::{serde_as, DisplayFromStr};

fn deserialize_object_id_to_hex<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let oid = Option::<ObjectId>::deserialize(deserializer)?;
    Ok(oid.map(|o| o.to_hex()))
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "_id", deserialize_with = "deserialize_object_id_to_hex", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub user_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub user_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
}