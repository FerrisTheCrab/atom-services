use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{
    router::{InternalRouter, Router},
    Service, ServiceInstance,
};

#[derive(Serialize, Deserialize)]
pub struct RegisterReq {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RegisterRes {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "error")]
    Error { reason: String },
}

impl RegisterRes {
    pub fn success(_: ()) -> Self {
        Self::Registered
    }

    pub fn failure(e: mongodb::error::Error) -> Self {
        Self::Error {
            reason: e
                .get_custom::<String>()
                .cloned()
                .unwrap_or(e.kind.to_string()),
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            RegisterRes::Registered => StatusCode::CREATED,
            RegisterRes::Error { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl InternalRouter {
    pub async fn register(instance: &ServiceInstance, payload: RegisterReq) -> RegisterRes {
        Service::register(instance, &payload.id)
            .await
            .map(RegisterRes::success)
            .unwrap_or_else(RegisterRes::failure)
    }
}

impl Router {
    pub async fn register(
        State(instance): State<ServiceInstance>,
        Json(payload): Json<RegisterReq>,
    ) -> (StatusCode, Json<RegisterRes>) {
        let res = InternalRouter::register(&instance, payload).await;
        (res.status(), Json(res))
    }
}
