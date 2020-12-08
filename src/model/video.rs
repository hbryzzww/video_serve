use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;

use crate::common::P;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: u32,
    pub cover: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
}

impl Responder for Video {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Video {
    pub async fn find_by_id(id: u32, pool: &MySqlPool) -> Result<Video> {
        let rec = sqlx::query!("SELECT id, cover, title, url FROM video WHERE id = ?", id)
            .fetch_one(&*pool)
            .await?;

        Ok(Video {
            id: rec.id,
            cover: rec.cover,
            title: rec.title,
            url: rec.url,
        })
    }

    pub async fn find_by_page(page: u8, size: u8, pool: &MySqlPool) -> Result<P<Video>> {
        let count = sqlx::query!("SELECT COUNT(1) as count FROM video")
            .fetch_one(pool)
            .await?;
        let mut videos = vec![];

        let offset = (page - 1) * size;

        let recs = sqlx::query!(
            "SELECT id, cover, title, url FROM video ORDER BY id DESC LIMIT ?,?",
            offset,
            size
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            videos.push(Video {
                id: rec.id,
                cover: rec.cover,
                title: rec.title,
                url: rec.url,
            });
        }

        Ok(P::new(count.count, videos))
    }
}
