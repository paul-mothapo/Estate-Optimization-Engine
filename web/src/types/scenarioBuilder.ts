import type { ApiJurisdiction } from './api'

export type ScenarioBuilderTaxpayerClass =
  | 'NaturalPerson'
  | 'Company'
  | 'Trust'
  | 'SpecialTrust'

export type ScenarioBuilderResidencyStatus = 'Resident' | 'NonResident'

export type ScenarioBuilderAssetDraft = {
  id: string
  name: string
  marketValueAmount: string
  baseCostAmount: string
  isLiquid: boolean
  situsInJurisdiction: boolean
  includedInEstateDuty: boolean
  includedInCgtDeemedDisposal: boolean
  bequeathedToSurvivingSpouse: boolean
  bequeathedToPbo: boolean
  qualifiesPrimaryResidenceExclusion: boolean
}

export type ScenarioBuilderDraft = {
  label: string
  jurisdiction: ApiJurisdiction
  taxYear: string
  taxpayerClass: ScenarioBuilderTaxpayerClass
  residencyStatus: ScenarioBuilderResidencyStatus
  marginalIncomeTaxRate: string
  debtsAndLoansAmount: string
  funeralCostsAmount: string
  administrationCostsAmount: string
  mastersOfficeFeesAmount: string
  conveyancingCostsAmount: string
  otherSettlementCostsAmount: string
  finalIncomeTaxDueAmount: string
  ongoingEstateIncomeTaxProvisionAmount: string
  additionalAllowableEstateTransferTaxDeductionsAmount: string
  portedEstateTaxExemptionAmount: string
  primaryResidenceCgtExclusionCapAmount: string
  executorFeeRate: string
  vatRate: string
  explicitExecutorFeeAmount: string
  externalLiquidityProceedsAmount: string
  cashReserveAmount: string
  assets: ScenarioBuilderAssetDraft[]
}
