#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;


use std::env;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;


mod cors;
mod handlers;
mod models;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    // TODO: Initialize pretty_env_logger
    // TODO: Initialize dotenv
    pretty_env_logger::init();
    dotenv().ok(); //load variables from .env file; eliminates need to add relative file path

    //Create a new PgPoolOptions instance with a maximum of 5 instances
    //Use dotenv to get the database URL
    //use 'unwrap' or 'expect' method to handle errors. If an error occurs at this stage, the server should be terminated
    // let pool = todo!();
    let pool = PgPoolOptions::new() //sets up a new PgPoolOptions instances, which is a sqlx v0.60 alias for PoolOptions,a configurable, asynchronous pool of SQLx database connections, specialized for Postgres.
        .max_connections(5) //Sets the maximum number of connections that this pool should maintain.
        .connect(&std::env::var("DATABASE_URL").expect("Did not load Postgres Database URL from .env file.")) //PUt in a string literal of desired variable from .env file; in this case, 'DATABASE_URL'
        .await //yield control of thread back to Runtime to await connection to Postgre database; without this line, 'pool' variable is not a Pool<Postgres> but of an unkonwn type
        .expect("Could not set up a PGPoolOptions instance");//error handling in case Postgres Pool is not initialized

    //Using SQLx, execute a SQL query that selects all the questions from the questions table
    //Use the 'unwrap' or 'expect' method to handle errors. This is just some test code to make sure we can connect to the database
    //let recs = todo!();
    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool) ////The method you want to call depends on how many rows you’re expecting, in this case, fetch_all() expects multiple rows from the database
        .await
        .expect("Unable to fetch all questions from the questions table.");

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro


    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer,
            ],
        )
        .attach(CORS)
}