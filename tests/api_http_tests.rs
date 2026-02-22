use crate::api::contracts::{ApiErrorCode, ApiErrorResponse, ApiEstateScenarioInput, ApiHealthResponse};
use crate::api::http::app;
use crate::core::domain::models::EstateScenarioInput;
use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("Failed to build request"),
        )
        .await
        .expect("Route call failed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body");
    let parsed: ApiHealthResponse =
        serde_json::from_slice(&body).expect("Failed to deserialize health response");
    assert_eq!(parsed.status, "ok");
}

#[tokio::test]
async fn calculate_endpoint_returns_validation_error_for_empty_assets() {
    let payload = ApiEstateScenarioInput::from(EstateScenarioInput::default());
    let body = serde_json::to_vec(&payload).expect("Failed to serialize payload");

    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/scenario/calculate")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .expect("Failed to build request"),
        )
        .await
        .expect("Route call failed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body");
    let parsed: ApiErrorResponse =
        serde_json::from_slice(&body).expect("Failed to deserialize error response");
    assert_eq!(parsed.code, ApiErrorCode::Validation);
    assert!(parsed
        .validation_issues
        .iter()
        .any(|issue| issue.field == "assets"));
}

#[tokio::test]
async fn rules_endpoint_returns_rule_selection_error_for_unsupported_year() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/v1/rules/south-africa/2017")
                .body(Body::empty())
                .expect("Failed to build request"),
        )
        .await
        .expect("Route call failed");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body");
    let parsed: ApiErrorResponse =
        serde_json::from_slice(&body).expect("Failed to deserialize error response");
    assert_eq!(parsed.code, ApiErrorCode::RuleSelection);
    assert!(parsed.message.contains("2017"));
}
