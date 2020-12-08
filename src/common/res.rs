use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct P<T> {
    total: i64,
    content: Vec<T>,
}

impl<T: Serialize> P<T> {
    pub fn new(total: i64, data: Vec<T>) -> Self {
        P {
            total: total,
            content: data,
        }
    }
}

impl<T: Serialize> Responder for P<T> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
