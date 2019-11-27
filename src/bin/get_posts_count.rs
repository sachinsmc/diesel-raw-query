extern crate diesel;
extern crate diesel_raw_query;

use diesel_raw_query::*;
use diesel::prelude::*;
use diesel::sql_query;

fn main() {
    let connection = establish_connection();
    let results = sql_query("SELECT * FROM POSTS")
        .execute(&connection)
        .ok().unwrap();
    println!("There are total {:?} posts", results);
}
