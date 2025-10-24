use actix_web::web;

use crate::controllers::user_controller::{create_user, get_user, login_user};
use crate::middlewares::auth_middleware::AuthMiddleware;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
        .route("/register", web::post().to(create_user))
        .route("/login", web::post().to(login_user))
        .service(
            web::scope("")
            .wrap(AuthMiddleware)
            .route("/get_user", web::get().to(get_user))
        )
    );
}