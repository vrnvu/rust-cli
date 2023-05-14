// TODO references
// https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
// https://github.com/launchbadge/sqlx/blob/main/examples/sqlite/todos/src/main.rs

use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Sqlite, SqlitePool};
use structopt::StructOpt;

/// This cli allows you to add todos to a sqlite db list
///
/// When a todo is complete you can mark it as done
#[derive(Debug, StructOpt)]
#[structopt()]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Lists all todos and show their state
    List,
    /// Adds a new todo to the list
    Add { description: String },
    /// Marks the todo with {id} as done
    Done { id: u32 },
}

#[derive(Debug, Clone, FromRow)]
struct Todo {
    id: u32,
    description: String,
}

const DB_URL: &'static str = "sqlite://todos.db";

#[tokio::main]
async fn main() -> Result<()> {
    match Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        true => {}
        false => Sqlite::create_database(DB_URL).await?,
    }

    let db = SqlitePool::connect(DB_URL).await?;

    create_todos(&db).await?;

    let opt = Args::from_args_safe()?;
    if let Some(cmd) = opt.cmd {
        match cmd {
            Command::List => list(&db).await?,
            Command::Add { description } => add(&db, description).await?,
            Command::Done { id } => done(&db, id).await?,
        }
    }

    Ok(())
}

async fn create_todos(db: &Pool<Sqlite>) -> Result<()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY NOT NULL, description VARCHAR(250) NOT NULL);")
    .execute(db)
    .await?;

    Ok(())
}

async fn list(db: &Pool<Sqlite>) -> Result<()> {
    println!("shows all todos from {DB_URL}");

    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(db)
        .await?;

    for todo in todos {
        println!("[{}, {}]", todo.id, todo.description);
    }

    Ok(())
}

async fn add(db: &Pool<Sqlite>, description: String) -> Result<()> {
    println!("adding {description} to {DB_URL}");

    sqlx::query("INSERT into todos (description) VALUES (?)")
        .bind(description)
        .execute(db)
        .await?;

    Ok(())
}

async fn done(_: &Pool<Sqlite>, id: u32) -> Result<()> {
    println!("removing {id} from {DB_URL}");

    Ok(())
}
