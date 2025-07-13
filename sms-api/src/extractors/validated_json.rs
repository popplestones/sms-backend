use std::ops::Deref;

use axum::{
    Json,
    body::Body,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::rejection::RequestRejection;

pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send + 'static,
    S: Send + Sync,
{
    type Rejection = RequestRejection;

    fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(|e| RequestRejection(StatusCode::BAD_REQUEST, e.to_string()))?;

            if let Err(validation_errors) = value.validate() {
                let errors = validation_errors
                    .field_errors()
                    .iter()
                    .map(|(field, errs)| {
                        let messages: Vec<String> = errs
                            .iter()
                            .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                            .collect();
                        (field.to_string(), serde_json::Value::from(messages))
                    })
                    .collect::<serde_json::Map<_, _>>();

                let error_body = Json(serde_json::json!({ "errors": errors }));

                return Err(RequestRejection(
                    axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    error_body.to_string(),
                ));
            }

            Ok(ValidatedJson(value))
        })
    }
}
