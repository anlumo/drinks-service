use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json},
    App, HttpServer, Responder,
};
use clap::Parser;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::chrono, Pool, Postgres, Row};
use urlencoding::encode;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello!")
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoryResponseEntry {
    date: chrono::NaiveDate,
    category: Option<i16>,
    count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoryResponse(Vec<HistoryResponseEntry>);

#[get("/history")]
async fn history(pool: Data<Pool<Postgres>>) -> actix_web::Result<Json<HistoryResponse>> {
    let mut rows = sqlx::query(
        "SELECT date, category, count FROM drinks, eancodes
        WHERE
            date BETWEEN (NOW() - interval '30 days') AND NOW()
            AND drinks.ean = eancodes.id",
    )
    .fetch(&**pool);

    let mut responses = Vec::new();

    while let Some(row) = rows.try_next().await.map_err(|err| {
        log::error!("Database: {err}");
        ErrorInternalServerError("Database")
    })? {
        let date = row.try_get("date").map_err(|err| {
            log::error!("Database: {err}");
            ErrorInternalServerError("Database")
        })?;
        let category = row.try_get("category").map_err(|err| {
            log::error!("Database: {err}");
            ErrorInternalServerError("Database")
        })?;
        let count = row.try_get("count").map_err(|err| {
            log::error!("Database: {err}");
            ErrorInternalServerError("Database")
        })?;
        responses.push(HistoryResponseEntry {
            date,
            category,
            count,
        });
    }

    Ok(Json(HistoryResponse(responses)))
}

#[derive(Debug, Serialize, Deserialize)]
struct RankingResponseEntry {
    name: String,
    total: i64,
    category: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct RankingResponseCategory {
    category: i16,
    entries: Vec<RankingResponseEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RankingResponse(Vec<RankingResponseCategory>);

#[get("/rankings")]
async fn rankings(pool: Data<Pool<Postgres>>) -> actix_web::Result<Json<RankingResponse>> {
    let mut category_data = Vec::with_capacity(5);
    for category in 0..5 {
        let mut rows = if category == 0 {
            sqlx::query(
                "SELECT
                name,
                SUM(count) AS total,
                category
            FROM drinks, eancodes
            WHERE drinks.date >= (CURRENT_DATE - INTERVAL '1 month')
                AND drinks.ean=eancodes.id
            GROUP BY drinks.ean,eancodes.name, eancodes.category
            ORDER BY total DESC LIMIT 10",
            )
        } else {
            sqlx::query(
                "SELECT
                    name,
                    SUM(count) AS total,
                    category
                FROM drinks, eancodes
                WHERE drinks.date >= (CURRENT_DATE - INTERVAL '1 month')
                    AND drinks.ean=eancodes.id
                    AND category = $1
                GROUP BY drinks.ean,eancodes.name, eancodes.category
                ORDER BY total DESC LIMIT 10",
            )
            .bind(category)
        }
        .fetch(&**pool);

        let mut entries = Vec::new();

        while let Some(row) = rows.try_next().await.map_err(|err| {
            log::error!("Database: {err}");
            ErrorInternalServerError("Database")
        })? {
            let name = row.try_get("name").map_err(|err| {
                log::error!("Database: {err}");
                ErrorInternalServerError("Database")
            })?;
            let total = row.try_get("total").map_err(|err| {
                log::error!("Database: {err}");
                ErrorInternalServerError("Database")
            })?;
            let category = row.try_get("category").map_err(|err| {
                log::error!("Database: {err}");
                ErrorInternalServerError("Database")
            })?;
            entries.push(RankingResponseEntry {
                name,
                total,
                category,
            });
        }

        category_data.push(RankingResponseCategory { category, entries })
    }

    Ok(Json(RankingResponse(category_data)))
}

#[derive(Parser)]
struct Args {
    #[clap(long, env)]
    postgres_host: String,
    #[clap(short = 'u', long, env)]
    postgres_user: String,
    #[clap(short = 'p', long, env)]
    postgres_password: String,
    #[clap(short = 'd', long, env)]
    postgres_database: String,
    #[clap(short = 'l', long, env)]
    listen: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let pool = Data::new(
        PgPoolOptions::new()
            .max_connections(3)
            .connect(&format!(
                "postgres://{}:{}@{}/{}",
                encode(&args.postgres_user),
                encode(&args.postgres_password),
                encode(&args.postgres_host),
                encode(&args.postgres_database)
            ))
            .await
            .expect("Connecting to database"),
    );

    log::info!("Listening on {}", args.listen);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(index)
            .service(history)
            .service(rankings)
    })
    .bind(args.listen)?
    .run()
    .await
}
