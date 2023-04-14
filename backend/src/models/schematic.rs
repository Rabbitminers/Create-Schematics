use crate::{
    config::db::Connection,
    schema::schematics::{self, dsl::*}, 
    models::user::User,
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

    pub fn update(schematic: SchematicDTO, user: User, conn: &Connection) -> Result<String, String> {
        unimplemented!()
    }

    fn insert_schematic(schematic: &Schematic, conn: &Connection) -> Result<(), String> {
        diesel::insert_into(schematics)
            .values(schematic)
            .execute(conn)
            .map_err(|err| format!("Failed to insert schematic: {:?}", err))?;
        Ok(())
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

    pub fn by_id(conn: &Connection, query_id: String) -> QueryResult<Option<Self>> {
        schematics.filter(schematics::id.eq(query_id)).first(conn).optional()
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