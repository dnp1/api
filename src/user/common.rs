use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExposedSession {
    pub user_id: Option<Uuid>,
}