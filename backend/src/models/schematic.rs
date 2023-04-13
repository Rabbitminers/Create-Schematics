use crate::{
    config::db::Connection,
    schema::schematics::{self, dsl::*}, 
    constants,
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

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

#[derive(Insertable)]
#[table_name = "schematics"]
pub struct SchematicDTO {
    pub title: String,
    pub description: String,
    pub author: i32,
    pub tags: String,
    pub display: String
}

impl Schematic {
    pub fn upload(schematic: SchematicDTO, conn: &Connection) -> Result<String, String> {
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
            title: schematic.title,
            description: schematic.description,
            author: schematic.author,
            tags: schematic.tags,
            display: schematic.display,
            date: now.naive_utc()
        };

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

    fn insert_schematic(schematic: &Schematic, conn: &Connection) -> Result<(), String> {
        diesel::insert_into(schematics)
            .values(schematic)
            .execute(conn)
            .map_err(|err| format!("Failed to insert schematic: {:?}", err))?;
        Ok(())
    }

    pub fn by_id(conn: &Connection, query_id: String) -> QueryResult<Option<Self>> {
        schematics.filter(schematics::id.eq(query_id)).first(conn).optional()
    }

    pub fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}