use lazy_static::lazy_static;
use std::net::SocketAddr;
use axum::{
    extract::Query,
    response::{Response, IntoResponse}, http::StatusCode, Json,
};
use serde::Deserialize;

pub async fn ping() -> &'static str {
    "pong"
}

lazy_static! {
    static ref CLIENT: crate::volo_gen::volo_gen::mini::redis::MiniRedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        crate::volo_gen::volo_gen::mini::redis::MiniRedisServiceClientBuilder::new("mini-redis")
            .address(addr)
            .build()
    };
}


#[derive(Debug, Deserialize)]
pub struct GetValueRequest {
    key: Option<String>,
}

pub async fn get_value(Query(request): Query<GetValueRequest>) -> Response {
    if let Some(key) = request.key {
        let req = crate::volo_gen::volo_gen::mini::redis::GetValueRequest {
            key: key.clone().into(),
        };
        let resp = CLIENT.get_value(req).await;
        match resp {
            Ok(resp) => {
                (StatusCode::OK, resp.value.to_string()).into_response()
            }
            Err(_) => {
                (StatusCode::NOT_FOUND, "key not found".to_string()).into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, "key is required".to_string()).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct SetValueRequest {
    key: Option<String>,
    value: Option<String>,
    expire: Option<u64>,
}

pub async fn set_value(Json(request): Json<SetValueRequest>) -> Response {
    if let Some(key) = request.key {
        if let Some(value) = request.value {
            let mut req = crate::volo_gen::volo_gen::mini::redis::SetValueRequest {
                key: key.clone().into(),
                value: value.clone().into(),
                expire_seconds: None,
            };
            if let Some(expire) = request.expire {
                req.expire_seconds = Some(expire as i32);
            }
            let resp = CLIENT.set_value(req).await;
            match resp {
                Ok(_) => {
                    (StatusCode::OK, "ok".to_string()).into_response()
                }
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string()).into_response()
                }
            }
        } else {
            (StatusCode::BAD_REQUEST, "value is required".to_string()).into_response()
        }
    } else {
        (StatusCode::BAD_REQUEST, "key is required".to_string()).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteValueRequest {
    key: Option<String>,
}

pub async fn delete_value(Query(request): Query<DeleteValueRequest>) -> Response {
    if let Some(key) = request.key {
        let req = crate::volo_gen::volo_gen::mini::redis::DeleteValueRequest {
            key: key.clone().into(),
        };
        let resp = CLIENT.delete_value(req).await;
        match resp {
            Ok(_) => {
                (StatusCode::OK, "ok".to_string()).into_response()
            }
            Err(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string()).into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, "key is required".to_string()).into_response()
    }
}