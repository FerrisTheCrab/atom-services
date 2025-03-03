#[cfg(feature = "core")]
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[cfg(feature = "core")]
use crate::{
    router::{InternalRouter, Router},
    Service, ServiceInstance,
};

#[derive(Serialize, Deserialize)]
pub struct ExistsReq {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExistsRes {
    #[serde(rename = "exists")]
    Exists { value: bool },
    #[serde(rename = "error")]
    Error { reason: String },
}

#[cfg(feature = "core")]
impl ExistsRes {
    pub fn success(value: bool) -> Self {
        Self::Exists { value }
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
            ExistsRes::Exists { .. } => StatusCode::OK,
            ExistsRes::Error { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "core")]
impl InternalRouter {
    pub async fn exists(instance: &ServiceInstance, payload: ExistsReq) -> ExistsRes {
        Service::exists(instance, &payload.id)
            .await
            .map(ExistsRes::success)
            .unwrap_or_else(ExistsRes::failure)
    }
}

#[cfg(feature = "core")]
impl Router {
    pub async fn exists(
        State(instance): State<ServiceInstance>,
        Json(payload): Json<ExistsReq>,
    ) -> (StatusCode, Json<ExistsRes>) {
        let res = InternalRouter::exists(&instance, payload).await;
        (res.status(), Json(res))
    }
}
