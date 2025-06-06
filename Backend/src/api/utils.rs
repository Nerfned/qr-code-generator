use axum::{http::StatusCode, response::Response};

pub fn not_found() -> Response {
    let mut response = Response::default();
    *response.status_mut() = StatusCode::NOT_FOUND;

    return response;
}

pub fn unauthorized() -> Response {
    let mut response = Response::default();
    *response.status_mut() = StatusCode::UNAUTHORIZED;

    return response;
}


pub fn conflict() -> Response {
    let mut response = Response::default();
    *response.status_mut() = StatusCode::CONFLICT;

    return response;
}