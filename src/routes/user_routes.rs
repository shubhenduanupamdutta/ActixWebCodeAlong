use actix_web::{middleware::from_fn, web};

use super::{handlers::user_handlers, middleware};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
            .service(user_handlers::user),
    );
}
