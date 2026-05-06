use axum::{
    body::{Body, Bytes},
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use http_body_util::BodyExt;

enum Direction {
    Request,
    Response(StatusCode),
}

const SENSITIVE_MASK: &str = "***";

pub async fn log_request_response(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let path = parts.uri.path().to_string();
    let bytes = buffer_and_print(Direction::Request, &path, body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print(Direction::Response(parts.status), &path, body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(
    direction: Direction,
    path: &str,
    body: B,
) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let message_type = match direction {
        Direction::Request => "request",
        Direction::Response(_) => "response",
    };

    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {message_type} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        match direction {
            Direction::Request => log_request(message_type, path, body),
            Direction::Response(status) => log_response(message_type, path, status, body),
        }
    };

    Ok(bytes)
}

fn log_request(message_type: &str, path: &str, message: &str) {
    if path.starts_with("/auth/login") {
        log_mask_value(message_type, path, None, message, "password");
    } else {
        tracing::info!(message_type, path, message);
    }
}

fn log_response(message_type: &str, path: &str, status: StatusCode, message: &str) {
    if status.is_server_error() {
        tracing::error!(message_type, path, status = status.as_u16(), message);
    } else if status.is_client_error() {
        tracing::warn!(message_type, path, status = status.as_u16(), message);
    } else {
        if path.starts_with("/auth/login") {
            log_mask_value(message_type, path, Some(status), message, "token");
        } else {
            tracing::info!(message_type, path, status = status.as_u16(), message);
        }
    }
}

fn log_mask_value(
    message_type: &str,
    path: &str,
    status: Option<StatusCode>,
    message: &str,
    field: &str,
) {
    if let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(message.as_bytes()) {
        if let Some(obj) = value.as_object_mut() {
            if let Some(field_value) = obj.get_mut(field) {
                if field_value.is_string() {
                    *field_value = SENSITIVE_MASK.into();

                    if let Some(status) = status {
                        tracing::info!(message_type, path, status = status.as_u16(), message = %value);
                    } else {
                        tracing::info!(message_type, path, message = %value);
                    }
                }
            }
        }
    }
}
