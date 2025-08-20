use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use std::future::Future;
use crate::common::error::AppError;


#[derive(Debug)]
pub struct ResourceId<T> {
    pub value: T,
}

impl<S> FromRequestParts<S> for ResourceId<i32>
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Path::<i32>::from_request_parts(parts, state)
            .await
            .map(|Path(id)| ResourceId { value: id })
            .map_err(AppError::from)
    }
}
