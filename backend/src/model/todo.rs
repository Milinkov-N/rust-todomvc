use sqlb::HasFields;

use crate::model::{self, db::Db};
use crate::security::UserCtx;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // creator id
    pub title: String,
    pub status: TodoStatus,
}

#[derive(sqlb::Fields, Debug, Default, Clone)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}

sqlb::bindable!(TodoStatus);

pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        let mut fields = data.fields();
        fields.push(("cid", 123).into());

        let sb = sqlb::insert()
            .table("todo")
            .data(fields)
            .returning(&["id", "cid", "title", "status"]);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        // let sql = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";

        // build the sqlx-query
        let sb = sqlb::select()
            .table("todo")
            .columns(&["id", "cid", "title", "status"]);

        //execute the query
        let todos = sb.fetch_all(db).await?;

        Ok(todos)
    }
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
