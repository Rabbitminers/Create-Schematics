use std::borrow::Borrow;

use crate::{
    config::db::Connection,
    models::schematic::Schematic,
    models::user::User,
    schema::comments::{self, dsl::*},   
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;


#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable)]
#[belongs_to(Schematic)]
#[table_name = "comments"]
pub struct Comment {
    pub id: String,
    pub schematic_id: String,
    pub rating: i32,
    pub body: String,
    pub author: i32,
    pub date: NaiveDateTime
}

#[derive(Insertable, AsChangeset)]
#[table_name = "comments"]
pub struct CommentDTO {
    pub schematic_id: String,
    pub rating: i32,
    pub body: String,
}


impl Comment {
    fn create(comment: CommentDTO, user_id: i32) -> Comment {
        return Comment {
            id: Self::generate_id(),
            schematic_id: comment.schematic_id,
            rating: comment.rating,
            body: comment.body,
            author: user_id,
            date: Utc::now().naive_utc()
        }
    }

    fn save_comment(
        new_comment: Comment,
        conn: &Connection
    ) -> QueryResult<usize> {
        diesel::insert_into(comments)
            .values(&new_comment)
            .execute(conn)
    }

    pub fn post(comment: CommentDTO, user_id: i32, conn: &Connection) -> Result<String, String> {
        match comments.filter(author.eq(user_id)).first::<Comment>(conn) {
            Ok(_) => {
                Err("You have already posted a comment on this schematic.".to_string())
            }
            Err(_) => {
                let new_comment = Comment::create(comment, user_id);
                let comment_id = new_comment.id.clone();
                Comment::save_comment(new_comment, conn);
                Ok(format!("Posted new comment with id: {}", comment_id))
            }
        }
    }

    pub fn delete(comment_id: String, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(comments.filter(id.eq(comment_id))).execute(conn)
    }

    pub fn update(comment_id: String, updated_comment: CommentDTO,  conn: &Connection) -> QueryResult<usize> {
        diesel::update(comments.find(comment_id))
            .set(&updated_comment)
            .execute(conn)
    }

    pub fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}