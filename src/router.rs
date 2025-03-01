use axum::routing::post;

use crate::ServiceInstance;

pub struct InternalRouter;
pub struct Router;

impl Router {
    pub fn get(instance: ServiceInstance) -> axum::Router {
        axum::Router::new()
            .route("/deregister", post(Router::deregister))
            .route("/register", post(Router::register))
            .route("/set", post(Router::set))
            .route("/show", post(Router::show))
            .route("/exists", post(Router::exists))
            .with_state(instance)
    }
}
