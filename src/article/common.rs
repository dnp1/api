use uuid::Uuid;
use postgres::rows;
//use serde;
use chrono::NaiveDateTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct Article {
    id: Uuid,
    title: String,
    publication_datetime: NaiveDateTime,
    edition_datetime: NaiveDateTime,
}

impl Article {
    pub fn from_row(row: &rows::Row) -> Article {
        return Article {
            id: row.get("id"),
            title: row.get("title"),
            publication_datetime: row.get("publication_datetime"),
            edition_datetime: row.get("edition_datetime")
        };
    }
}