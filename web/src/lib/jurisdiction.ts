import type { ApiJurisdiction } from '../types/api'

export const DEFAULT_JURISDICTIONS: ApiJurisdiction[] = ['SouthAfrica']

export function toJurisdictionPathToken(jurisdiction: ApiJurisdiction): string {
  switch (jurisdiction) {
    case 'SouthAfrica':
      return 'south-africa'
    case 'UsNewYork':
      return 'us-new-york'
    case 'UsTexas':
      return 'us-texas'
    case 'UsCalifornia':
      return 'us-california'
    case 'UsFlorida':
      return 'us-florida'
    case 'UsMinnesota':
      return 'us-minnesota'
  }
}
