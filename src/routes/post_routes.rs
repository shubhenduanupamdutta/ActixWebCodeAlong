use actix_web::{middleware::from_fn, web};

use super::{handlers::post_handlers, middleware};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            // Secure Posts
            web::scope("/post")
                .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
                .service(post_handlers::create_post)
                .service(post_handlers::get_my_posts),
        )
        .service(web::scope("/post")); // Unsecure Post Apis
}
