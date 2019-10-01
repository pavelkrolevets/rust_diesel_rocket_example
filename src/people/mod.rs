#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::people;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "people"]

pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub profession: String,
    pub salary: i32,
}
