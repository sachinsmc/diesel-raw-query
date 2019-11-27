# diesel-raw-query
This repository contains code for Diesel ORM with Postgres. 

This file src/bin/get_posts_count.rs shows raw query execution with diesel.

```rust,no_run
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

```


# Installation

Install Rust and Diesel CLI : 

`$ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly`

`$ cargo install diesel_cli`

After installtion update run

`$ cp .example.env .env`

and update DATABASE_URL in .env file.

Run : `$ diesel setup` for checking db creds and creating database if not exist.

This code uses posts table, you have to create it to create in your database run 

`$ diesel migration run`

---
Run : `$ cargo run --bin get_posts_count` If zero records then it will show anything There are total 0 posts.

If you want to add some records run : `$ cargo run --bin write_post`





