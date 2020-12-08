use actix_web::web;

use crate::controller::{
    detail::video,
    home::home,
    list::{index, partition},
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
    cfg.service(index);
    cfg.service(partition);
    cfg.service(video);
}
