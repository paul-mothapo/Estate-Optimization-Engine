use crate::api::contracts::{
    ApiErrorCode, ApiErrorResponse, ApiEstateScenarioInput, ApiHealthResponse, ApiJurisdiction,
    ApiJurisdictionTaxRuleRegistryResponse, ApiOptimizedScenario, ApiScenarioResult,
    ApiTaxRuleRegistryEntry, ApiValidationIssue, ApiVersionedJurisdictionTaxRuleSet,
};
use crate::api::handler::{
    calculate_single_scenario_contract, get_jurisdiction_tax_rule_registry_contract,
    list_supported_jurisdictions_contract, list_tax_rule_registry_entries_contract,
    optimize_candidate_scenarios_contract, resolve_latest_tax_rules_contract,
    resolve_tax_rules_for_year_contract,
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};

type HttpError = (StatusCode, Json<ApiErrorResponse>);
type HttpResult<T> = Result<Json<T>, HttpError>;

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/jurisdictions", get(list_jurisdictions))
        .route("/v1/rules/registry", get(list_registry_entries))
        .route(
            "/v1/rules/registry/{jurisdiction}",
            get(get_registry_for_jurisdiction),
        )
        .route("/v1/rules/latest/{jurisdiction}", get(resolve_latest_rules))
        .route(
            "/v1/rules/{jurisdiction}/{tax_year}",
            get(resolve_rules_for_year),
        )
        .route("/v1/scenario/calculate", post(calculate_scenario))
        .route("/v1/scenario/optimize", post(optimize_scenarios))
}

pub async fn run(bind_addr: &str) -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    axum::serve(listener, app()).await
}

async fn health() -> Json<ApiHealthResponse> {
    Json(ApiHealthResponse {
        status: "ok".to_string(),
    })
}

async fn list_jurisdictions() -> Json<Vec<ApiJurisdiction>> {
    Json(list_supported_jurisdictions_contract())
}

async fn list_registry_entries() -> Json<Vec<ApiTaxRuleRegistryEntry>> {
    Json(list_tax_rule_registry_entries_contract())
}

async fn get_registry_for_jurisdiction(
    Path(jurisdiction): Path<String>,
) -> HttpResult<ApiJurisdictionTaxRuleRegistryResponse> {
    let jurisdiction = parse_jurisdiction(&jurisdiction).map_err(api_error_to_http)?;

    match get_jurisdiction_tax_rule_registry_contract(jurisdiction) {
        Some(response) => Ok(Json(response)),
        None => Err(not_found_response(
            "No rule registry found for requested jurisdiction",
        )),
    }
}

async fn resolve_latest_rules(Path(jurisdiction): Path<String>) -> HttpResult<ApiVersionedJurisdictionTaxRuleSet> {
    let jurisdiction = parse_jurisdiction(&jurisdiction).map_err(api_error_to_http)?;
    Ok(Json(resolve_latest_tax_rules_contract(jurisdiction)))
}

async fn resolve_rules_for_year(
    Path((jurisdiction, tax_year)): Path<(String, u16)>,
) -> HttpResult<ApiVersionedJurisdictionTaxRuleSet> {
    let jurisdiction = parse_jurisdiction(&jurisdiction).map_err(api_error_to_http)?;
    resolve_tax_rules_for_year_contract(jurisdiction, tax_year)
        .map(Json)
        .map_err(api_error_to_http)
}

async fn calculate_scenario(
    Json(input): Json<ApiEstateScenarioInput>,
) -> HttpResult<ApiScenarioResult> {
    calculate_single_scenario_contract(input)
        .map(Json)
        .map_err(api_error_to_http)
}

async fn optimize_scenarios(
    Json(candidates): Json<Vec<ApiEstateScenarioInput>>,
) -> HttpResult<Option<ApiOptimizedScenario>> {
    optimize_candidate_scenarios_contract(candidates)
        .map(Json)
        .map_err(api_error_to_http)
}

fn parse_jurisdiction(raw: &str) -> Result<ApiJurisdiction, ApiErrorResponse> {
    match raw.to_ascii_lowercase().as_str() {
        "south-africa" | "south_africa" | "southafrica" | "za" => Ok(ApiJurisdiction::SouthAfrica),
        _ => Err(ApiErrorResponse {
            code: ApiErrorCode::Validation,
            message: format!("Unsupported jurisdiction path value '{raw}'"),
            validation_issues: vec![ApiValidationIssue {
                field: "jurisdiction".to_string(),
                message: "Use one of: south-africa, south_africa, southafrica, za".to_string(),
            }],
        }),
    }
}

fn api_error_to_http(error: ApiErrorResponse) -> HttpError {
    let status = match error.code {
        ApiErrorCode::Validation => StatusCode::BAD_REQUEST,
        ApiErrorCode::RuleSelection => StatusCode::UNPROCESSABLE_ENTITY,
        ApiErrorCode::Computation => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, Json(error))
}

fn not_found_response(message: &str) -> HttpError {
    (
        StatusCode::NOT_FOUND,
        Json(ApiErrorResponse {
            code: ApiErrorCode::Computation,
            message: message.to_string(),
            validation_issues: Vec::new(),
        }),
    )
}
