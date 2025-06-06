use actix_web::web;

use super::handlers::home_handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home")
            .service(home_handlers::greet)
            .service(home_handlers::test),
    );
}
