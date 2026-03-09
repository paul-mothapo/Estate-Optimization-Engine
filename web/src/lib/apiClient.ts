import {
  isLegalDocumentRoutingError,
  safeJsonParse,
} from './document'
import { logIntakeError, logIntakeEvent } from './intakeLogger'
import type {
  ApiCalculateResponse,
  ApiErrorResponse,
  ApiEstateDocumentAnalysisResponse,
  ApiHealthResponse,
  ApiIngestResponse,
  ApiJurisdiction,
  ApiJurisdictionTaxRuleRegistryResponse,
  ApiOptimizedScenario,
  ApiResponse,
  ApiScenarioDocumentFormat,
  ApiScenarioResult,
  ApiTaxRuleRegistryEntry,
  ApiVersionedJurisdictionTaxRuleSet,
  RunMode,
} from '../types/api'

type ParsedJsonResponse<T> = {
  ok: boolean
  status: number
  statusText: string
  rawText: string
  payload: T | ApiErrorResponse | null
}

export type DocumentWorkflowSuccess =
  | {
      kind: 'ingest'
      rawText: string
      payload: ApiIngestResponse
    }
  | {
      kind: 'calculate'
      rawText: string
      payload: ApiCalculateResponse
    }
  | {
      kind: 'analysis'
      rawText: string
      payload: ApiEstateDocumentAnalysisResponse
    }

export type DocumentWorkflowFailure = {
  message: string
  rawText: string
  payload: ApiErrorResponse | null
}

export type JsonSuccess<T> = {
  ok: true
  rawText: string
  payload: T
}

export type JsonFailure = {
  ok: false
  message: string
  rawText: string
  payload: ApiErrorResponse | null
}

async function parseJsonResponse<T>(response: Response): Promise<ParsedJsonResponse<T>> {
  const rawText = (await response.text()).trim()
  const payload = rawText.length > 0 ? safeJsonParse<T | ApiErrorResponse>(rawText) : null

  return {
    ok: response.ok,
    status: response.status,
    statusText: response.statusText,
    rawText,
    payload,
  }
}

function resolveFailureMessage(
  fallback: string,
  payload: ApiErrorResponse | null,
  status: number,
  statusText: string,
): string {
  if (payload?.message) {
    return payload.message
  }
  return `${fallback} (${status} ${statusText})`
}

function isApiErrorPayload(value: unknown): value is ApiErrorResponse {
  return value !== null && typeof value === 'object' && 'message' in value
}

export async function submitJsonRequest<T>(
  endpoint: string,
  init: RequestInit | undefined,
  fallbackMessage: string,
): Promise<JsonSuccess<T> | JsonFailure> {
  logIntakeEvent('json_request_started', {
    endpoint,
    method: init?.method ?? 'GET',
  })
  const response = await fetch(endpoint, init)
  const parsed = await parseJsonResponse<T>(response)
  logIntakeEvent('json_response_received', {
    endpoint,
    method: init?.method ?? 'GET',
    ok: parsed.ok,
    status: parsed.status,
    status_text: parsed.statusText,
    raw_length: parsed.rawText.length,
  })

  if (!parsed.ok) {
    const apiError = isApiErrorPayload(parsed.payload) ? parsed.payload : null
    logIntakeError('json_request_failed', {
      endpoint,
      message: apiError?.message ?? fallbackMessage,
      status: parsed.status,
    })

    return {
      ok: false,
      message: resolveFailureMessage(
        fallbackMessage,
        apiError,
        parsed.status,
        parsed.statusText,
      ),
      rawText: parsed.rawText,
      payload: apiError,
    }
  }

  if (!parsed.payload) {
    return {
      ok: false,
      message: resolveFailureMessage(
        'Server returned an empty response',
        null,
        parsed.status,
        parsed.statusText,
      ),
      rawText: parsed.rawText,
      payload: null,
    }
  }

  if (isApiErrorPayload(parsed.payload)) {
    return {
      ok: false,
      message: parsed.payload.message,
      rawText: parsed.rawText,
      payload: parsed.payload,
    }
  }

  return {
    ok: true,
    rawText: parsed.rawText,
    payload: parsed.payload,
  }
}

export async function submitDocumentWorkflow(args: {
  documentName: string
  format: ApiScenarioDocumentFormat
  mode: RunMode
  uploadPayload: { document_content: string; document_content_base64?: string }
}): Promise<DocumentWorkflowSuccess | DocumentWorkflowFailure> {
  const { documentName, format, mode, uploadPayload } = args
  const endpoint =
    mode === 'calculate'
      ? '/v1/scenario/document/calculate'
      : '/v1/scenario/ingest'

  logIntakeEvent('document_workflow_started', {
    document_name: documentName,
    format,
    mode,
    endpoint,
    text_length: uploadPayload.document_content.length,
    base64_length: uploadPayload.document_content_base64?.length ?? 0,
  })

  const response = await fetch(endpoint, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      format,
      ...uploadPayload,
    }),
  })
  const parsed = await parseJsonResponse<ApiResponse>(response)
  logIntakeEvent('document_workflow_response', {
    document_name: documentName,
    endpoint,
    ok: parsed.ok,
    status: parsed.status,
    status_text: parsed.statusText,
    raw_length: parsed.rawText.length,
  })

  if (!parsed.ok) {
    const apiError = isApiErrorPayload(parsed.payload) ? parsed.payload : null

    if (apiError && isLegalDocumentRoutingError(apiError)) {
      logIntakeEvent('document_workflow_rerouted', {
        document_name: documentName,
        from_endpoint: endpoint,
        to_endpoint: '/v1/estate/documents/analyze',
        reason: apiError.message,
      })
      const analysisResponse = await fetch('/v1/estate/documents/analyze', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({
          documents: [
            {
              document_name: documentName,
              format,
              ...uploadPayload,
            },
          ],
        }),
      })
      const analysisParsed =
        await parseJsonResponse<ApiEstateDocumentAnalysisResponse>(analysisResponse)
      logIntakeEvent('document_analysis_response', {
        document_name: documentName,
        ok: analysisParsed.ok,
        status: analysisParsed.status,
        status_text: analysisParsed.statusText,
        raw_length: analysisParsed.rawText.length,
      })

      if (!analysisParsed.ok || !analysisParsed.payload) {
        const analysisError = isApiErrorPayload(analysisParsed.payload)
          ? analysisParsed.payload
          : null
        logIntakeError('document_analysis_failed', {
          document_name: documentName,
          message: analysisError?.message ?? 'Estate document analysis failed',
          status: analysisParsed.status,
        })
        return {
          message: resolveFailureMessage(
            'Estate document analysis failed',
            analysisError,
            analysisParsed.status,
            analysisParsed.statusText,
          ),
          rawText: analysisParsed.rawText,
          payload: analysisError,
        }
      }

      if (isApiErrorPayload(analysisParsed.payload)) {
        return {
          message: analysisParsed.payload.message,
          rawText: analysisParsed.rawText,
          payload: analysisParsed.payload,
        }
      }

      return {
        kind: 'analysis',
        rawText: analysisParsed.rawText,
        payload: analysisParsed.payload,
      }
    }

    logIntakeError('document_workflow_failed', {
      document_name: documentName,
      endpoint,
      message: apiError?.message ?? 'Unable to process document',
      status: parsed.status,
    })

    return {
      message: resolveFailureMessage(
        'Unable to process document',
        apiError,
        parsed.status,
        parsed.statusText,
      ),
      rawText: parsed.rawText,
      payload: apiError,
    }
  }

  if (!parsed.payload) {
    return {
      message: resolveFailureMessage(
        'Server returned an empty response',
        null,
        parsed.status,
        parsed.statusText,
      ),
      rawText: parsed.rawText,
      payload: null,
    }
  }

  if (isApiErrorPayload(parsed.payload)) {
    return {
      message: parsed.payload.message,
      rawText: parsed.rawText,
      payload: parsed.payload,
    }
  }

  if (mode === 'calculate') {
    logIntakeEvent('document_workflow_completed', {
      document_name: documentName,
      kind: 'calculate',
      scenarios: (parsed.payload as ApiCalculateResponse).scenarios.length,
      results: (parsed.payload as ApiCalculateResponse).results.length,
    })
    return {
      kind: 'calculate',
      rawText: parsed.rawText,
      payload: parsed.payload as ApiCalculateResponse,
    }
  }

  logIntakeEvent('document_workflow_completed', {
    document_name: documentName,
    kind: 'ingest',
    scenarios: (parsed.payload as ApiIngestResponse).scenarios.length,
  })
  return {
    kind: 'ingest',
    rawText: parsed.rawText,
    payload: parsed.payload as ApiIngestResponse,
  }
}

export function createJsonPostInit(payload: unknown): RequestInit {
  return {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  }
}

export type RulesPayload =
  | ApiJurisdiction[]
  | ApiTaxRuleRegistryEntry[]
  | ApiJurisdictionTaxRuleRegistryResponse
  | ApiVersionedJurisdictionTaxRuleSet

export async function submitScenarioCalculation(payload: Record<string, unknown>) {
  return submitJsonRequest<ApiScenarioResult>(
    '/v1/scenario/calculate',
    createJsonPostInit(payload),
    'Scenario calculation failed',
  )
}

export async function submitScenarioOptimization(payload: unknown[]) {
  return submitJsonRequest<ApiOptimizedScenario | null>(
    '/v1/scenario/optimize',
    createJsonPostInit(payload),
    'Scenario optimization failed',
  )
}

export async function submitEstateDocumentAnalysis(
  documents: Array<{
    document_name: string
    format: ApiScenarioDocumentFormat
    document_content: string
    document_content_base64?: string
  }>,
) {
  logIntakeEvent('estate_batch_analysis_started', {
    documents: documents.map((document) => ({
      name: document.document_name,
      format: document.format,
      text_length: document.document_content.length,
      base64_length: document.document_content_base64?.length ?? 0,
    })),
  })
  return submitJsonRequest<ApiEstateDocumentAnalysisResponse>(
    '/v1/estate/documents/analyze',
    createJsonPostInit({ documents }),
    'Estate document analysis failed',
  )
}

export async function fetchRulesPayload(endpoint: string) {
  return submitJsonRequest<RulesPayload>(endpoint, undefined, 'Rules request failed')
}

export async function fetchHealthPayload(endpoint: '/health' | '/health/db') {
  return submitJsonRequest<ApiHealthResponse>(endpoint, undefined, 'Health request failed')
}
