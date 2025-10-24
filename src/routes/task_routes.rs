use actix_web::web;

use crate::controllers::task_controller::{create_task, delete_task, get_task, get_tasks, update_task};
use crate::middlewares::auth_middleware::AuthMiddleware;

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .wrap(AuthMiddleware)
            .route("", web::post().to(create_task))
            .route("", web::get().to(get_tasks))
            .route("/{task_id}", web::get().to(get_task))
            .route("/{task_id}", web::put().to(update_task))
            .route("/{task_id}", web::delete().to(delete_task))
    );
}