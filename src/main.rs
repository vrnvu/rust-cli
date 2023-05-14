// TODO references
// https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
// https://github.com/launchbadge/sqlx/blob/main/examples/sqlite/todos/src/main.rs

use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
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
    Done { id: u64 },
}

const DB_URL: &'static str = "sqlite://todos.db";

#[tokio::main]
async fn main() -> Result<()> {
    match Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        true => {}
        false => Sqlite::create_database(DB_URL).await?,
    }

    let db = SqlitePool::connect(DB_URL).await?;

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

async fn list(db: &Pool<Sqlite>) -> Result<()> {
    println!("shows all todos from {DB_URL}");
    let recs = sqlx::query(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#,
    )
    .fetch_all(db)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }
    Ok(())
}

async fn add(db: &Pool<Sqlite>, description: String) -> Result<()> {
    println!("adding {description} to {DB_URL}");
    Ok(())
}

async fn done(db: &Pool<Sqlite>, id: u64) -> Result<()> {
    println!("removing {id} from {DB_URL}");
    Ok(())
}
