use axum::{
    body::Body,
    extract::{FromRequest, Request},
};
use sms_core::models::SmsSubmission;

use crate::{
    auth::extractor::AuthenticatedUser,
    extractors::{utils::split_request_body, validated_json::ValidatedJson},
    rejection::RequestRejection,
};

pub struct SmsRequest {
    pub user: AuthenticatedUser,
    pub data: SmsSubmission,
}

const BODY_LIMIT: usize = 8192; // 8KB

impl<S> FromRequest<S> for SmsRequest
where
    S: Send + Sync + 'static,
{
    type Rejection = RequestRejection;

    fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            let (req1, req2) = split_request_body(req, BODY_LIMIT).await?;

            let user = AuthenticatedUser::from_request(req1, state).await?;
            let ValidatedJson(data) =
                ValidatedJson::<SmsSubmission>::from_request(req2, state).await?;

            Ok(SmsRequest { user, data })
        })
    }
}
