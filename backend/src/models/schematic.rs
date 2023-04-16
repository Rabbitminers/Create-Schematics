use crate::{
    config::db::Connection,
    schema::schematics::{self, dsl::*}, 
    models::user::User,
    constants,
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::{filters::SchematicFilter, response::Page, pagination::SortingAndPaging};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Insertable)]
pub struct Schematic {
    pub id: String,
    pub title: String,
    pub description: String,
    pub author: i32,
    pub tags: String,
    pub display: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "schematics"]
pub struct SchematicDTO {
    pub title: String,
    pub description: String,
    pub tags: String,
    pub display: String
}

impl Schematic {
    pub fn upload(schematic: SchematicDTO, user: User, conn: &Connection) -> Result<String, String> {
        match Schematic::new(schematic, user.id) {
            Ok(new_schematic) => {
                match Schematic::insert_schematic(&new_schematic, conn) {
                    Ok(()) => {
                        Ok(format!(
                            "Successfully added new schematic with id: {}",
                            new_schematic.id
                        ))
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn find_by_id(query_id: String, conn: &Connection) -> QueryResult<Option<Self>> {
        schematics.filter(schematics::id.eq(query_id)).first(conn).optional()
    }

    pub fn find_all(page_num: i64, conn: &Connection) -> QueryResult<Vec<Schematic>> {
        schematics.order(title.asc())
            .limit(constants::DEFAULT_PER_PAGE)
            .offset((page_num - 1) * constants::DEFAULT_PER_PAGE)
            .load::<Schematic>(conn)
    }

    pub fn filter(filter: SchematicFilter, conn: &Connection) -> QueryResult<Page<Schematic>> {
        let mut query = schematics::table.into_boxed();
        
        if let Some(i) = filter.title {
            query = query.filter(title.like(format!("%{}%", i)));
        }

        if let Some(i) = filter.author {
            query = query.filter(author.eq(i));
        }

        if let Some(i) = filter.tags {
            let tag_list: Vec<&str> = i.split(",").map(|t| t.trim()).collect();
            for tag in tag_list {
                query = query.filter(tags.like(format!("%{}%", tag)));
            }
        }

        if let Some(i) = filter.date {
            query = query.filter(date.eq(i));
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
            .load_and_count_items::<Schematic>(conn)
    }


    fn insert_schematic(schematic: &Schematic, conn: &Connection) -> Result<(), String> {
        diesel::insert_into(schematics)
            .values(schematic)
            .execute(conn)
            .map_err(|err| format!("Failed to insert schematic: {:?}", err))?;
        Ok(())
    }

    pub fn update(schematic_id: String, updated_schematic: SchematicDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(schematics.find(schematic_id))
            .set(&updated_schematic)
            .execute(conn)
    }

    pub fn delete(schematic_id: String, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(schematics.find(schematic_id)).execute(conn)
    }


    fn new(schematic: SchematicDTO, user_id: i32) -> Result<Schematic, String> {
        if schematic.title.is_empty() || schematic.description.is_empty() {
            return Err(constants::MESSAGE_EMPTY.to_string())
        }
        if schematic.title.len() > 48 || schematic.description.len() > 1024 {
            return Err(constants::MESSAGE_TOO_LONG.to_string());
        }

        let schematic_id = Schematic::generate_id();
        let now = Utc::now();

        let new_schematic = Schematic {
            id: schematic_id,
            author: user_id,
            date: now.naive_utc(),
            ..schematic.into()
        };

        Ok(new_schematic)
    }

    pub fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}

impl From<SchematicDTO> for Schematic {
    fn from(dto: SchematicDTO) -> Self {
        Self {
            id: Schematic::generate_id(),
            title: dto.title,
            description: dto.description,
            author: 0,
            tags: dto.tags,
            display: dto.display,
            date: Utc::now().naive_utc(),
        }
    }
}