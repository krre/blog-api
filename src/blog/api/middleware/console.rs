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

pub async fn log_request_response(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print(Direction::Request, body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print(Direction::Response(parts.status), body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: Direction, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let direction_descr = match direction {
        Direction::Request => "request",
        Direction::Response(_) => "response",
    };

    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction_descr} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        match direction {
            Direction::Request => tracing::info!(messsge_type = direction_descr, message = body),
            Direction::Response(status) => tracing::info!(
                messsge_type = direction_descr,
                status = status.as_u16(),
                message = body
            ),
        }
    };

    Ok(bytes)
}
