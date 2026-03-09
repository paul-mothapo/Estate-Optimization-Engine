export type RunMode = 'ingest' | 'calculate'
export type StatusTone = 'idle' | 'busy' | 'success' | 'error'
export type ApiScenarioDocumentFormat = 'Json' | 'Txt' | 'Csv' | 'Docx' | 'Pdf'

export type ApiIssue = {
  field: string
  message: string
}

export type ApiErrorResponse = {
  code: string
  message: string
  validation_issues: ApiIssue[]
}

export type ApiEstateDocumentAnalysisResponse = {
  detections: Array<{
    document_index: number
    detected_document_types: string[]
    warnings: string[]
  }>
  checklist: Array<{
    requirement_id: string
    status: string
  }>
  missing_required_document_types: string[]
  readiness_score: number
}

export type ApiCombinedTax = {
  total_tax_liability_amount: number
}

export type ApiLiquidity = {
  liquidity_gap_amount: number
  liquidity_surplus_amount: number
}

export type ApiScenarioResult = {
  combined_tax: ApiCombinedTax
  liquidity: ApiLiquidity
}

export type ApiIngestResponse = {
  scenarios: unknown[]
}

export type ApiCalculateResponse = {
  scenarios: unknown[]
  results: ApiScenarioResult[]
}

export type ApiResponse = ApiIngestResponse | ApiCalculateResponse

export type Workflow =
  | 'document'
  | 'scenario'
  | 'optimize'
  | 'estate-docs'
  | 'rules'
  | 'health'

export type ApiJurisdiction =
  | 'SouthAfrica'
  | 'UsNewYork'
  | 'UsTexas'
  | 'UsCalifornia'
  | 'UsFlorida'
  | 'UsMinnesota'

export type ApiHealthResponse = {
  status: string
}

export type ApiLiquidityRiskBand = 'Low' | 'Moderate' | 'High' | 'Critical'

export type ApiScenarioScore = {
  tax_burden_ratio: number
  liquidity_cover_ratio: number
  liquidity_risk_band: ApiLiquidityRiskBand
  composite_score: number
}

export type ApiOptimizedScenario = {
  index: number
  input: Record<string, unknown>
  result: ApiScenarioResult
  score: ApiScenarioScore
}

export type ApiTaxRuleRegistryEntry = {
  jurisdiction: ApiJurisdiction
  tax_year: number
  version_id: string
  effective_from: string
  effective_to: string | null
  is_latest_for_jurisdiction: boolean
}

export type ApiJurisdictionTaxRuleRegistryResponse = {
  jurisdiction: ApiJurisdiction
  versions: ApiTaxRuleRegistryEntry[]
  supported_tax_year_from: number
  supported_tax_year_to: number
  latest_version_id: string
}

export type ApiVersionedJurisdictionTaxRuleSet = {
  jurisdiction: ApiJurisdiction
  supported_tax_year: number
  version: {
    version_id: string
    effective_from: string
    effective_to: string | null
  }
  rules: Record<string, unknown>
}
