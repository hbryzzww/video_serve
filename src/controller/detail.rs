use crate::model::Video;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use sqlx::mysql::MySqlPool;

#[get("/video/{id}")]
pub async fn video(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
    let id: u32 = req.match_info().query("id").parse().unwrap();

    let result = Video::find_by_id(id, pool.get_ref()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        _ => HttpResponse::BadRequest().body("Error trying to read all viodes from database"),
    }
}
