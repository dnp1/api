use uuid::Uuid;
use postgres::rows;
use chrono::NaiveDateTime;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: Uuid,
    user_id: Uuid,
    publication_datetime: NaiveDateTime,
    edition_datetime: NaiveDateTime,
}

impl Comment {
    pub fn from_row(row: &rows::Row) -> Comment {
        return Comment {
            id: row.get("id"),
            user_id: row.get("user_id"),
            publication_datetime: row.get("publication_datetime"),
            edition_datetime: row.get("edition_datetime")
        };
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    content: String
}

impl Content {
    pub fn from_row(row: &rows::Row) -> Content {
        return Content {
            content: row.get("content"),
        };
    }
}