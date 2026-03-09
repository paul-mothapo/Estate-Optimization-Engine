import type { ApiJurisdiction } from '../types/api'
import type {
  ScenarioBuilderAssetDraft,
  ScenarioBuilderDraft,
} from '../types/scenarioBuilder'

let assetCounter = 0

function nextAssetId(): string {
  assetCounter += 1
  return `asset-${assetCounter}`
}

export function createScenarioAssetDraft(
  overrides: Partial<ScenarioBuilderAssetDraft> = {},
): ScenarioBuilderAssetDraft {
  return {
    id: nextAssetId(),
    name: '',
    marketValueAmount: '0',
    baseCostAmount: '0',
    isLiquid: true,
    situsInJurisdiction: true,
    includedInEstateDuty: true,
    includedInCgtDeemedDisposal: true,
    bequeathedToSurvivingSpouse: false,
    bequeathedToPbo: false,
    qualifiesPrimaryResidenceExclusion: false,
    ...overrides,
  }
}

export function createScenarioDraft(
  label = 'Scenario',
  jurisdiction: ApiJurisdiction = 'SouthAfrica',
): ScenarioBuilderDraft {
  return {
    label,
    jurisdiction,
    taxYear: '2025',
    taxpayerClass: 'NaturalPerson',
    residencyStatus: 'Resident',
    marginalIncomeTaxRate: '0.45',
    debtsAndLoansAmount: '350000',
    funeralCostsAmount: '30000',
    administrationCostsAmount: '55000',
    mastersOfficeFeesAmount: '8000',
    conveyancingCostsAmount: '24000',
    otherSettlementCostsAmount: '12000',
    finalIncomeTaxDueAmount: '0',
    ongoingEstateIncomeTaxProvisionAmount: '0',
    additionalAllowableEstateTransferTaxDeductionsAmount: '0',
    portedEstateTaxExemptionAmount: '0',
    primaryResidenceCgtExclusionCapAmount: '2000000',
    executorFeeRate: '0.035',
    vatRate: '0.15',
    explicitExecutorFeeAmount: '',
    externalLiquidityProceedsAmount: '300000',
    cashReserveAmount: '150000',
    assets: [
      createScenarioAssetDraft({
        name: 'Primary Residence',
        marketValueAmount: '5000000',
        baseCostAmount: '3200000',
        isLiquid: false,
        qualifiesPrimaryResidenceExclusion: true,
      }),
      createScenarioAssetDraft({
        name: 'Investment Portfolio',
        marketValueAmount: '1800000',
        baseCostAmount: '1250000',
      }),
    ],
  }
}

function parseNumber(value: string): number {
  const normalized = value.trim().replace(/,/g, '')
  if (normalized.length === 0) {
    return 0
  }
  const parsed = Number(normalized)
  return Number.isFinite(parsed) ? parsed : 0
}

export function scenarioDraftToApiPayload(draft: ScenarioBuilderDraft) {
  return {
    jurisdiction: draft.jurisdiction,
    tax_year: Math.max(0, Math.trunc(parseNumber(draft.taxYear))),
    taxpayer_class: draft.taxpayerClass,
    residency_status: draft.residencyStatus,
    marginal_income_tax_rate: parseNumber(draft.marginalIncomeTaxRate),
    assets: draft.assets.map((asset) => ({
      name: asset.name.trim() || 'Unnamed Asset',
      market_value_amount: parseNumber(asset.marketValueAmount),
      base_cost_amount: parseNumber(asset.baseCostAmount),
      is_liquid: asset.isLiquid,
      situs_in_jurisdiction: asset.situsInJurisdiction,
      included_in_estate_duty: asset.includedInEstateDuty,
      included_in_cgt_deemed_disposal: asset.includedInCgtDeemedDisposal,
      bequeathed_to_surviving_spouse: asset.bequeathedToSurvivingSpouse,
      bequeathed_to_pbo: asset.bequeathedToPbo,
      qualifies_primary_residence_exclusion: asset.qualifiesPrimaryResidenceExclusion,
    })),
    debts_and_loans_amount: parseNumber(draft.debtsAndLoansAmount),
    funeral_costs_amount: parseNumber(draft.funeralCostsAmount),
    administration_costs_amount: parseNumber(draft.administrationCostsAmount),
    masters_office_fees_amount: parseNumber(draft.mastersOfficeFeesAmount),
    conveyancing_costs_amount: parseNumber(draft.conveyancingCostsAmount),
    other_settlement_costs_amount: parseNumber(draft.otherSettlementCostsAmount),
    final_income_tax_due_amount: parseNumber(draft.finalIncomeTaxDueAmount),
    ongoing_estate_income_tax_provision_amount: parseNumber(
      draft.ongoingEstateIncomeTaxProvisionAmount,
    ),
    additional_allowable_estate_transfer_tax_deductions_amount: parseNumber(
      draft.additionalAllowableEstateTransferTaxDeductionsAmount,
    ),
    ported_estate_tax_exemption_amount: parseNumber(draft.portedEstateTaxExemptionAmount),
    primary_residence_cgt_exclusion_cap_amount: parseNumber(
      draft.primaryResidenceCgtExclusionCapAmount,
    ),
    executor_fee_rate: parseNumber(draft.executorFeeRate),
    vat_rate: parseNumber(draft.vatRate),
    explicit_executor_fee_amount:
      draft.explicitExecutorFeeAmount.trim().length > 0
        ? parseNumber(draft.explicitExecutorFeeAmount)
        : null,
    external_liquidity_proceeds_amount: parseNumber(draft.externalLiquidityProceedsAmount),
    cash_reserve_amount: parseNumber(draft.cashReserveAmount),
  }
}
