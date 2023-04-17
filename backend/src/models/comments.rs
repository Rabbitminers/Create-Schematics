use crate::{
    config::db::Connection,
    models::{
        schematic::Schematic,
        filters::CommentFilter,
        response::Page,
        user::User
    },
    schema::comments::{self, dsl::*}, constants,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, sqlite::Sqlite};
use uuid::Uuid;

use super::pagination::SortingAndPaging;


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
    pub rating: i32,
    pub body: String,
}


impl Comment {
    fn create(comment: CommentDTO, parent: String, user_id: i32) -> Result<Comment, String> {
        if (1..=5).contains(&comment.rating) {
            Ok(Comment {
                id: Self::generate_id(),
                schematic_id: parent,
                rating: comment.rating,
                body: comment.body,
                author: user_id,
                date: Utc::now().naive_utc()
            })
        } else {
            Err(constants::MESSAGE_INVALID_RATING.to_string())
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

    pub fn filter(parent: String, filter: CommentFilter, conn: &Connection) -> QueryResult<Page<Comment>> {
        let mut query = comments::table
                .into_boxed::<Sqlite>()
                .filter(schematic_id.eq(parent));

        if let Some(i) = filter.rating {
            query = query.filter(rating.eq(i));
        }

        if let Some(i) = filter.author {
            query = query.filter(author.eq(i));
        }

        query
            .paginate(
            filter.page_num.unwrap_or(crate::constants::DEFAULT_PAGE_NUM),
            )
            .per_page(
                filter.page_size.unwrap_or(crate::constants::DEFAULT_PER_PAGE),
            )
            .sort(
        filter.sort_by.unwrap_or(crate::constants::EMPTY_STR.to_string()),
 filter.sort_direction.unwrap_or(crate::constants::EMPTY_STR.to_string()),
            )
            .load_and_count_items::<Comment>(conn)
    }

    pub fn post(comment: CommentDTO, parent: String, user_id: i32, conn: &Connection) -> Result<String, String> {
        match comments.filter(author.eq(user_id)).first::<Comment>(conn) {
            Ok(_) => {
                Err(constants::MESSAGE_DUPLICATE_COMMENT.to_string())
            }
            Err(_) => {
                match Self::create(comment,  parent, user_id) {
                    Ok(new_comment) => {
                        let comment_id = new_comment.id.clone();
                        Comment::save_comment(new_comment, conn);
                        Ok(format!("Posted new comment with id: {}", comment_id))
                    }
                    Err(e) => Err(e)
                }
            }
        }
    }

    pub fn get_by_id(comment_id: &String, conn: &Connection) -> Option<Comment> {
        comments.filter(id.eq(comment_id))
            .first(conn)
            .optional()
            .unwrap_or(None)
    }

    pub fn delete(comment_id: String, user: User, conn: &Connection) -> Result<QueryResult<usize>, String> {
        match Self::get_by_id(&comment_id, conn) {
            Some(c) if c.author == user.id => {
                Ok(diesel::delete(comments.find(comment_id)).execute(conn))
            }
            Some(_) => Err(constants::MESSAGE_CAN_NOT_DELETE_COMMENT.to_string()),
            None => Err(constants::MESSAGE_COMMENT_NOT_FOUND.to_string()),
        }
    }

    pub fn update(comment_id: String, user: User, updated_comment: CommentDTO,  conn: &Connection) -> Result<QueryResult<usize>, String> {
        match Self::get_by_id(&comment_id, conn) {
            Some(c) if c.author == user.id => {
                Ok(diesel::update(comments.find(comment_id))
                    .set(&updated_comment)
                    .execute(conn)
                )
            }
            Some(_) => Err(constants::MESSAGE_CAN_NOT_MODIFY_COMMENT.to_string()),
            None => Err(constants::MESSAGE_COMMENT_NOT_FOUND.to_string()),
        }
    }

    pub fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn is_owned_by(&self, user: &User) -> bool {
        self.author == user.id
    }
}