#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use surrealdb::{
    RecordId, Surreal,
    engine::local::{Db, Mem},
    opt::Resource,
};

static DB: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    DB.connect::<Mem>(()).await?;
    DB.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Option<Record> = DB
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    // We don't care about the response in this case
    // so we are just going to use `Resource::from`
    // to let the compiler return `surrealdb::Value`
    DB.update(Resource::from(("person", "jaime")))
        .merge(Responsibility { marketing: true })
        .await?;

    // Select all people records
    let people: Vec<Record> = DB.select("person").await?;
    dbg!(people);

    Ok(())
}
