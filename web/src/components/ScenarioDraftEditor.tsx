import type { ApiJurisdiction } from '../types/api'
import type {
  ScenarioBuilderAssetDraft,
  ScenarioBuilderDraft,
  ScenarioBuilderResidencyStatus,
  ScenarioBuilderTaxpayerClass,
} from '../types/scenarioBuilder'

type ScenarioDraftEditorProps = {
  draft: ScenarioBuilderDraft
  onDraftChange: <K extends keyof ScenarioBuilderDraft>(
    field: K,
    value: ScenarioBuilderDraft[K],
  ) => void
  onAssetChange: (
    assetId: string,
    field: keyof ScenarioBuilderAssetDraft,
    value: string | boolean,
  ) => void
  onAddAsset: () => void
  onRemoveAsset: (assetId: string) => void
  canRemoveAsset: boolean
}

const JURISDICTIONS: ApiJurisdiction[] = [
  'SouthAfrica',
  'UsNewYork',
  'UsTexas',
  'UsCalifornia',
  'UsFlorida',
  'UsMinnesota',
]

const TAXPAYER_CLASSES: ScenarioBuilderTaxpayerClass[] = [
  'NaturalPerson',
  'Company',
  'Trust',
  'SpecialTrust',
]

const RESIDENCY_STATUSES: ScenarioBuilderResidencyStatus[] = ['Resident', 'NonResident']

function renderToggle(
  label: string,
  checked: boolean,
  onChange: (checked: boolean) => void,
) {
  return (
    <label className="toggle-item">
      <input type="checkbox" checked={checked} onChange={(event) => onChange(event.target.checked)} />
      <span>{label}</span>
    </label>
  )
}

function ScenarioDraftEditor({
  draft,
  onDraftChange,
  onAssetChange,
  onAddAsset,
  onRemoveAsset,
  canRemoveAsset,
}: ScenarioDraftEditorProps) {
  return (
    <div className="draft-editor">
      <div className="section-header">
        <h3>{draft.label}</h3>
      </div>

      <div className="form-grid">
        <label className="field-stack">
          <span>Jurisdiction</span>
          <select
            className="control-input"
            value={draft.jurisdiction}
            onChange={(event) => onDraftChange('jurisdiction', event.target.value as ApiJurisdiction)}
          >
            {JURISDICTIONS.map((item) => (
              <option key={item} value={item}>
                {item}
              </option>
            ))}
          </select>
        </label>

        <label className="field-stack">
          <span>Tax Year</span>
          <input
            className="control-input"
            value={draft.taxYear}
            inputMode="numeric"
            onChange={(event) => onDraftChange('taxYear', event.target.value)}
          />
        </label>

        <label className="field-stack">
          <span>Taxpayer Class</span>
          <select
            className="control-input"
            value={draft.taxpayerClass}
            onChange={(event) =>
              onDraftChange('taxpayerClass', event.target.value as ScenarioBuilderTaxpayerClass)
            }
          >
            {TAXPAYER_CLASSES.map((item) => (
              <option key={item} value={item}>
                {item}
              </option>
            ))}
          </select>
        </label>

        <label className="field-stack">
          <span>Residency Status</span>
          <select
            className="control-input"
            value={draft.residencyStatus}
            onChange={(event) =>
              onDraftChange(
                'residencyStatus',
                event.target.value as ScenarioBuilderResidencyStatus,
              )
            }
          >
            {RESIDENCY_STATUSES.map((item) => (
              <option key={item} value={item}>
                {item}
              </option>
            ))}
          </select>
        </label>

        <label className="field-stack">
          <span>Marginal Tax Rate</span>
          <input
            className="control-input"
            value={draft.marginalIncomeTaxRate}
            inputMode="decimal"
            onChange={(event) => onDraftChange('marginalIncomeTaxRate', event.target.value)}
          />
        </label>

        <label className="field-stack">
          <span>Executor Fee Rate</span>
          <input
            className="control-input"
            value={draft.executorFeeRate}
            inputMode="decimal"
            onChange={(event) => onDraftChange('executorFeeRate', event.target.value)}
          />
        </label>

        <label className="field-stack">
          <span>VAT Rate</span>
          <input
            className="control-input"
            value={draft.vatRate}
            inputMode="decimal"
            onChange={(event) => onDraftChange('vatRate', event.target.value)}
          />
        </label>
      </div>

      <div className="section-header compact">
        <h4>Estate Costs and Liquidity</h4>
      </div>

      <div className="form-grid">
        <label className="field-stack">
          <span>Debts and Loans</span>
          <input className="control-input" value={draft.debtsAndLoansAmount} inputMode="decimal" onChange={(event) => onDraftChange('debtsAndLoansAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Funeral Costs</span>
          <input className="control-input" value={draft.funeralCostsAmount} inputMode="decimal" onChange={(event) => onDraftChange('funeralCostsAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Administration Costs</span>
          <input className="control-input" value={draft.administrationCostsAmount} inputMode="decimal" onChange={(event) => onDraftChange('administrationCostsAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Master's Office Fees</span>
          <input className="control-input" value={draft.mastersOfficeFeesAmount} inputMode="decimal" onChange={(event) => onDraftChange('mastersOfficeFeesAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Conveyancing Costs</span>
          <input className="control-input" value={draft.conveyancingCostsAmount} inputMode="decimal" onChange={(event) => onDraftChange('conveyancingCostsAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Other Settlement Costs</span>
          <input className="control-input" value={draft.otherSettlementCostsAmount} inputMode="decimal" onChange={(event) => onDraftChange('otherSettlementCostsAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Final Income Tax Due</span>
          <input className="control-input" value={draft.finalIncomeTaxDueAmount} inputMode="decimal" onChange={(event) => onDraftChange('finalIncomeTaxDueAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Ongoing Estate Income Tax Provision</span>
          <input className="control-input" value={draft.ongoingEstateIncomeTaxProvisionAmount} inputMode="decimal" onChange={(event) => onDraftChange('ongoingEstateIncomeTaxProvisionAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Additional Allowable Deductions</span>
          <input className="control-input" value={draft.additionalAllowableEstateTransferTaxDeductionsAmount} inputMode="decimal" onChange={(event) => onDraftChange('additionalAllowableEstateTransferTaxDeductionsAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Ported Estate Exemption</span>
          <input className="control-input" value={draft.portedEstateTaxExemptionAmount} inputMode="decimal" onChange={(event) => onDraftChange('portedEstateTaxExemptionAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Primary Residence CGT Cap</span>
          <input className="control-input" value={draft.primaryResidenceCgtExclusionCapAmount} inputMode="decimal" onChange={(event) => onDraftChange('primaryResidenceCgtExclusionCapAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Explicit Executor Fee</span>
          <input className="control-input" value={draft.explicitExecutorFeeAmount} inputMode="decimal" onChange={(event) => onDraftChange('explicitExecutorFeeAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>External Liquidity Proceeds</span>
          <input className="control-input" value={draft.externalLiquidityProceedsAmount} inputMode="decimal" onChange={(event) => onDraftChange('externalLiquidityProceedsAmount', event.target.value)} />
        </label>
        <label className="field-stack">
          <span>Cash Reserve</span>
          <input className="control-input" value={draft.cashReserveAmount} inputMode="decimal" onChange={(event) => onDraftChange('cashReserveAmount', event.target.value)} />
        </label>
      </div>

      <div className="section-header compact">
        <h4>Assets</h4>
        <button className="btn btn-secondary" type="button" onClick={onAddAsset}>
          Add Asset
        </button>
      </div>

      <div className="asset-list">
        {draft.assets.map((asset) => (
          <article className="asset-card" key={asset.id}>
            <div className="section-header compact">
              <h4>{asset.name.trim() || 'Untitled Asset'}</h4>
              {canRemoveAsset && (
                <button className="btn btn-secondary" type="button" onClick={() => onRemoveAsset(asset.id)}>
                  Remove
                </button>
              )}
            </div>

            <div className="form-grid">
              <label className="field-stack">
                <span>Asset Name</span>
                <input className="control-input" value={asset.name} onChange={(event) => onAssetChange(asset.id, 'name', event.target.value)} />
              </label>
              <label className="field-stack">
                <span>Market Value</span>
                <input className="control-input" value={asset.marketValueAmount} inputMode="decimal" onChange={(event) => onAssetChange(asset.id, 'marketValueAmount', event.target.value)} />
              </label>
              <label className="field-stack">
                <span>Base Cost</span>
                <input className="control-input" value={asset.baseCostAmount} inputMode="decimal" onChange={(event) => onAssetChange(asset.id, 'baseCostAmount', event.target.value)} />
              </label>
            </div>

            <div className="toggle-grid">
              {renderToggle('Liquid Asset', asset.isLiquid, (value) => onAssetChange(asset.id, 'isLiquid', value))}
              {renderToggle('In Jurisdiction', asset.situsInJurisdiction, (value) => onAssetChange(asset.id, 'situsInJurisdiction', value))}
              {renderToggle('Included in Estate Duty', asset.includedInEstateDuty, (value) => onAssetChange(asset.id, 'includedInEstateDuty', value))}
              {renderToggle('Included in CGT Deemed Disposal', asset.includedInCgtDeemedDisposal, (value) => onAssetChange(asset.id, 'includedInCgtDeemedDisposal', value))}
              {renderToggle('Bequeathed to Surviving Spouse', asset.bequeathedToSurvivingSpouse, (value) => onAssetChange(asset.id, 'bequeathedToSurvivingSpouse', value))}
              {renderToggle('Bequeathed to PBO', asset.bequeathedToPbo, (value) => onAssetChange(asset.id, 'bequeathedToPbo', value))}
              {renderToggle('Primary Residence Exclusion', asset.qualifiesPrimaryResidenceExclusion, (value) => onAssetChange(asset.id, 'qualifiesPrimaryResidenceExclusion', value))}
            </div>
          </article>
        ))}
      </div>
    </div>
  )
}

export default ScenarioDraftEditor
