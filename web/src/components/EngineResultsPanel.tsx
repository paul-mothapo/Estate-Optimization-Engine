import { formatCurrency } from '../lib/document'
import type {
  ApiEstateDocumentAnalysisResponse,
  ApiOptimizedScenario,
  ApiScenarioResult,
} from '../types/api'

type EngineResultsPanelProps = {
  summaryText: string
  insightFacts: Array<{ label: string; value: string }>
  insightBadges: string[]
  documentResults: ApiScenarioResult[]
  singleScenarioResult: ApiScenarioResult | null
  optimizedScenario: ApiOptimizedScenario | null
  estateAnalysis: ApiEstateDocumentAnalysisResponse | null
}

function EngineResultsPanel({
  summaryText,
  insightFacts,
  insightBadges,
  documentResults,
  singleScenarioResult,
  optimizedScenario,
  estateAnalysis,
}: EngineResultsPanelProps) {
  return (
    <section className="results-shell">
      <article className="summary-card">
        <p className="summary-label">Workspace Summary</p>
        <p className="summary">{summaryText}</p>
      </article>

      {insightFacts.length > 0 && (
        <article className="result-card">
          <h2>Overview</h2>
          <dl>
            {insightFacts.map((fact) => (
              <div key={fact.label}>
                <dt>{fact.label}</dt>
                <dd>{fact.value}</dd>
              </div>
            ))}
          </dl>
          {insightBadges.length > 0 && (
            <div className="file-chip-row">
              {insightBadges.map((item) => (
                <span key={item} className="file-chip">
                  {item}
                </span>
              ))}
            </div>
          )}
        </article>
      )}

      {singleScenarioResult && (
        <article className="result-card">
          <h2>Scenario Result</h2>
          <dl>
            <div>
              <dt>Total Tax</dt>
              <dd>{formatCurrency(singleScenarioResult.combined_tax?.total_tax_liability_amount)}</dd>
            </div>
            <div>
              <dt>Liquidity Gap</dt>
              <dd>{formatCurrency(singleScenarioResult.liquidity?.liquidity_gap_amount)}</dd>
            </div>
            <div>
              <dt>Liquidity Surplus</dt>
              <dd>{formatCurrency(singleScenarioResult.liquidity?.liquidity_surplus_amount)}</dd>
            </div>
          </dl>
        </article>
      )}

      {optimizedScenario && (
        <article className="result-card">
          <h2>Optimized Scenario</h2>
          <dl>
            <div>
              <dt>Selected Index</dt>
              <dd>{optimizedScenario.index}</dd>
            </div>
            <div>
              <dt>Composite Score</dt>
              <dd>{optimizedScenario.score.composite_score.toFixed(4)}</dd>
            </div>
            <div>
              <dt>Risk Band</dt>
              <dd>{optimizedScenario.score.liquidity_risk_band}</dd>
            </div>
            <div>
              <dt>Total Tax</dt>
              <dd>{formatCurrency(optimizedScenario.result.combined_tax?.total_tax_liability_amount)}</dd>
            </div>
          </dl>
        </article>
      )}

      {estateAnalysis && (
        <article className="result-card">
          <h2>Estate Intake Readiness</h2>
          <dl>
            <div>
              <dt>Readiness</dt>
              <dd>{Math.round(estateAnalysis.readiness_score * 100)}%</dd>
            </div>
            <div>
              <dt>Detected Documents</dt>
              <dd>{estateAnalysis.detections.length}</dd>
            </div>
            <div>
              <dt>Checklist Items</dt>
              <dd>{estateAnalysis.checklist.length}</dd>
            </div>
            <div>
              <dt>Missing Required Types</dt>
              <dd>{estateAnalysis.missing_required_document_types.length}</dd>
            </div>
          </dl>
          <div className="file-chip-row">
            {estateAnalysis.missing_required_document_types.slice(0, 8).map((item) => (
              <span key={item} className="file-chip">
                {item}
              </span>
            ))}
          </div>
          <div className="checklist-list">
            {estateAnalysis.checklist.slice(0, 6).map((item) => (
              <div className="checklist-row" key={item.requirement_id}>
                <strong>{item.requirement_id}</strong>
                <span>{item.status}</span>
              </div>
            ))}
          </div>
        </article>
      )}

      {documentResults.length > 0 && (
        <div className="results-grid">
          {documentResults.map((result, index) => (
            <article className="result-card" key={index}>
              <h2>Document Scenario {index + 1}</h2>
              <dl>
                <div>
                  <dt>Total Tax</dt>
                  <dd>{formatCurrency(result.combined_tax?.total_tax_liability_amount)}</dd>
                </div>
                <div>
                  <dt>Liquidity Gap</dt>
                  <dd>{formatCurrency(result.liquidity?.liquidity_gap_amount)}</dd>
                </div>
                <div>
                  <dt>Liquidity Surplus</dt>
                  <dd>{formatCurrency(result.liquidity?.liquidity_surplus_amount)}</dd>
                </div>
              </dl>
            </article>
          ))}
        </div>
      )}
    </section>
  )
}

export default EngineResultsPanel
