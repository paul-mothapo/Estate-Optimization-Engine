import { DEFAULT_JURISDICTIONS } from '../lib/jurisdiction'
import type { ApiJurisdiction } from '../types/api'

type RulesExplorerFormProps = {
  jurisdictions: ApiJurisdiction[]
  selectedJurisdiction: ApiJurisdiction
  taxYear: string
  isSubmitting: boolean
  onJurisdictionChange: (value: ApiJurisdiction) => void
  onTaxYearChange: (value: string) => void
  onLoadJurisdictions: () => void
  onLoadRegistry: () => void
  onLoadJurisdictionRegistry: () => void
  onLoadLatestRules: () => void
  onLoadRulesByYear: () => void
}

function RulesExplorerForm({
  jurisdictions,
  selectedJurisdiction,
  taxYear,
  isSubmitting,
  onJurisdictionChange,
  onTaxYearChange,
  onLoadJurisdictions,
  onLoadRegistry,
  onLoadJurisdictionRegistry,
  onLoadLatestRules,
  onLoadRulesByYear,
}: RulesExplorerFormProps) {
  const options = jurisdictions.length > 0 ? jurisdictions : DEFAULT_JURISDICTIONS

  return (
    <>
      <label className="field-label" htmlFor="jurisdiction">
        Jurisdiction
      </label>
      <select
        id="jurisdiction"
        className="control-input"
        value={selectedJurisdiction}
        onChange={(event) => onJurisdictionChange(event.target.value as ApiJurisdiction)}
      >
        {options.map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>

      <label className="field-label" htmlFor="tax-year">
        Tax Year
      </label>
      <input
        id="tax-year"
        className="control-input"
        value={taxYear}
        onChange={(event) => onTaxYearChange(event.target.value)}
        inputMode="numeric"
      />

      <p className="field-help">
        Query jurisdiction support, rule registry entries, and effective rule sets.
      </p>

      <div className="actions">
        <button className="btn btn-primary" type="button" onClick={onLoadJurisdictions} disabled={isSubmitting}>
          Load Jurisdictions
        </button>
        <button className="btn btn-secondary" type="button" onClick={onLoadRegistry} disabled={isSubmitting}>
          Registry
        </button>
        <button className="btn btn-secondary" type="button" onClick={onLoadJurisdictionRegistry} disabled={isSubmitting}>
          Registry by Jurisdiction
        </button>
        <button className="btn btn-secondary" type="button" onClick={onLoadLatestRules} disabled={isSubmitting}>
          Latest Rules
        </button>
        <button className="btn btn-secondary" type="button" onClick={onLoadRulesByYear} disabled={isSubmitting}>
          Rules by Year
        </button>
      </div>
    </>
  )
}

export default RulesExplorerForm
