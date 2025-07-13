use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::auth::jwt::{self, Claims};

pub struct AuthenticatedUser(pub Claims);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync + 'static,
{
    type Rejection = (StatusCode, String);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl futures::Future<Output = Result<Self, <Self as FromRequestParts<S>>::Rejection>>
    + std::marker::Send {
        let fut = async move {
            let TypedHeader(auth) =
                TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                    .await
                    .map_err(|_| {
                        (
                            StatusCode::UNAUTHORIZED,
                            "Missing Authorization header".to_string(),
                        )
                    })?;

            let token = auth.0.token();
            let token_data = jwt::decode_jwt(token)
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid JWT".to_string()))?;

            Ok(AuthenticatedUser(token_data.claims))
        };
        Box::pin(fut)
    }
}
