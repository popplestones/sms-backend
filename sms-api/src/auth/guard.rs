use axum::http::StatusCode;

use crate::{
    auth::{extractor::AuthenticatedUser, permissions::Permission},
    rejection::RequestRejection,
};

pub fn permission_required(
    user: &AuthenticatedUser,
    permission: Permission,
) -> Result<(), RequestRejection> {
    if user.can(permission.clone()) {
        Ok(())
    } else {
        Err(RequestRejection(
            StatusCode::FORBIDDEN,
            format!("Permission denied: {permission:?}"),
        ))
    }
}
