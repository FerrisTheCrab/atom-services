#[cfg(feature = "core")]
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[cfg(feature = "core")]
use crate::{
    router::{InternalRouter, Router},
    Service, ServiceInstance,
};

#[derive(Serialize, Deserialize)]
pub struct DeregisterReq {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DeregisterRes {
    #[serde(rename = "deregistered")]
    Deregistered,
    #[serde(rename = "error")]
    Error { reason: String },
}

#[cfg(feature = "core")]
impl DeregisterRes {
    pub fn success(_: ()) -> Self {
        Self::Deregistered
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
            DeregisterRes::Deregistered => StatusCode::OK,
            DeregisterRes::Error { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "core")]
impl InternalRouter {
    pub async fn deregister(instance: &ServiceInstance, payload: DeregisterReq) -> DeregisterRes {
        Service::deregister(instance, &payload.id)
            .await
            .map(DeregisterRes::success)
            .unwrap_or_else(DeregisterRes::failure)
    }
}

#[cfg(feature = "core")]
impl Router {
    pub async fn deregister(
        State(instance): State<ServiceInstance>,
        Json(payload): Json<DeregisterReq>,
    ) -> (StatusCode, Json<DeregisterRes>) {
        let res = InternalRouter::deregister(&instance, payload).await;
        (res.status(), Json(res))
    }
}
