/// The `ApiUser` middleware is used to identify the user in the API.
///
///
/// # Actual implementation
/// The actual implementation simply take the user id from the `X-EntityUid` header.
///
/// # What's next
/// In the future, the entity uid must be securly retrieved from another source.
/// A common implementation will be to use openid connect provider like keycloak, auth0, etc.
///
/// Doing it this way implies not many changes in the backend code:
/// - the `ApiUser` struct must be updated to match the authenticated user data
/// - the `FromRequestParts` implementation must be updated to retrieve the user id from the authentication token and validate it
///
/// The frontend of course will have to be updated accordingly as well (ex: send the token instead of the user id and perform authentication flow with the oidc provider)
/// but this is beyond the scope of this middleware.
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ApiUser {
    uid: EntityUid,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ApiUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let uid = parts
            .headers
            .get("X-Entity-Uid")
            .ok_or_else(|| ApiError::Forbidden("No X-Entity-Uid header".to_string()))
            .and_then(|value| {
                value
                    .to_str()
                    .ok()
                    .and_then(|value| value.parse().ok())
                    .ok_or_else(|| ApiError::Forbidden("Invalid X-Entity-Uid header".to_string()))
            })?;
        Ok(ApiUser { uid })
    }
}

impl Entity for ApiUser {
    fn uid(&self) -> EntityUid {
        self.uid
    }
}
