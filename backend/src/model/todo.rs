use crate::model;
use crate::model::db::Db;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // creator id
    pub title: String,
}

pub struct TodoMac;

impl TodoMac {
    pub async fn list(db: &Db) -> Result<Vec<Todo>, model::Error> {
        let sql = "SELECT id, cid, title FROM todo ORDER BY id DESC";

        // build the sqlx-query
        let query = sqlx::query_as(&sql);

        //execute the query
        let todos = query.fetch_all(db).await?;

        Ok(todos)
    }
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
