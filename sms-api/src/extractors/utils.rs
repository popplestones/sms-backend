use axum::{body::Body, extract::Request, http::StatusCode};

use crate::rejection::RequestRejection;

pub fn split_request_body(
    req: Request<Body>,
    limit: usize,
) -> impl Future<Output = Result<(Request<Body>, Request<Body>), RequestRejection>> + Send {
    async move {
        let (parts, body) = req.into_parts();
        let bytes = axum::body::to_bytes(body, limit).await.map_err(|_| {
            RequestRejection(StatusCode::INTERNAL_SERVER_ERROR, "Body too large".into())
        })?;

        // Rebuild two requests from the same body
        let req1 = Request::from_parts(parts.clone(), Body::from(bytes.clone()));
        let req2 = Request::from_parts(parts, Body::from(bytes));

        Ok((req1, req2))
    }
}
