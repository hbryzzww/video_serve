use actix_web::web;

use crate::controller::{
    detail::video,
    home::home,
    list::{catgory, index, partition, search},
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
    cfg.service(index);
    cfg.service(partition);
    cfg.service(video);
    cfg.service(catgory);
    cfg.service(search);
}
