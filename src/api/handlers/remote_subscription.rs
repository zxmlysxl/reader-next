use crate::api::auth::AuthContext;
use crate::api::AppState;
use crate::error::error::{ApiResponse, AppError};
use crate::model::remote_subscription::RemoteSubscription;
use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RemoteSubscriptionUrlParam {
    pub url: String,
}

pub async fn list_remote_subscriptions(
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<Json<ApiResponse<Vec<RemoteSubscription>>>, AppError> {
    let user_ns = state
        .user_service
        .resolve_user_ns_with_override(auth.access_token(), auth.secure_key(), auth.user_ns())
        .await
        .map_err(|_| AppError::BadRequest("NEED_LOGIN".to_string()))?;
    let subs = state.remote_subscription_repo.list(&user_ns).await?;
    Ok(Json(ApiResponse::ok(subs)))
}

pub async fn add_remote_subscription(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(param): Json<RemoteSubscriptionUrlParam>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let user_ns = state
        .user_service
        .resolve_user_ns_with_override(auth.access_token(), auth.secure_key(), auth.user_ns())
        .await
        .map_err(|_| AppError::BadRequest("NEED_LOGIN".to_string()))?;
    let url = param.url.trim();
    if url.is_empty() {
        return Err(AppError::BadRequest("url is required".to_string()));
    }
    state.remote_subscription_repo.upsert(&user_ns, url).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({ "saved": true }))))
}

pub async fn remove_remote_subscription(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(param): Json<RemoteSubscriptionUrlParam>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let user_ns = state
        .user_service
        .resolve_user_ns_with_override(auth.access_token(), auth.secure_key(), auth.user_ns())
        .await
        .map_err(|_| AppError::BadRequest("NEED_LOGIN".to_string()))?;
    state.remote_subscription_repo.delete(&user_ns, &param.url).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({ "deleted": true }))))
}

pub async fn update_remote_subscription_synced(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(param): Json<RemoteSubscriptionUrlParam>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let user_ns = state
        .user_service
        .resolve_user_ns_with_override(auth.access_token(), auth.secure_key(), auth.user_ns())
        .await
        .map_err(|_| AppError::BadRequest("NEED_LOGIN".to_string()))?;
    state.remote_subscription_repo.update_last_synced(&user_ns, &param.url).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({ "updated": true }))))
}
