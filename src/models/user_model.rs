use serde::{Serialize, Deserialize, Deserializer};
use mongodb::bson::oid::ObjectId;

fn deserialize_object_id_to_hex<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let oid = Option::<ObjectId>::deserialize(deserializer)?;
    Ok(oid.map(|o| o.to_hex()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", deserialize_with = "deserialize_object_id_to_hex", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: String,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}