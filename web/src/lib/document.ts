import type {
  ApiErrorResponse,
  ApiScenarioDocumentFormat,
} from '../types/api'
import { logIntakeEvent } from './intakeLogger'

export function isLegalDocumentRoutingError(error: ApiErrorResponse): boolean {
  if (error.message.toLowerCase().includes('legal/tax estate document')) {
    return true
  }
  return error.validation_issues.some((issue) =>
    issue.message.includes('/v1/estate/documents/analyze'),
  )
}

export function safeJsonParse<T>(raw: string): T | null {
  try {
    return JSON.parse(raw) as T
  } catch {
    return null
  }
}

export function inferDocumentFormat(file: File): ApiScenarioDocumentFormat | null {
  const extension = file.name.toLowerCase().split('.').pop()
  if (extension === 'json') return 'Json'
  if (extension === 'txt') return 'Txt'
  if (extension === 'csv') return 'Csv'
  if (extension === 'docx') return 'Docx'
  if (extension === 'pdf') return 'Pdf'

  const mime = file.type.toLowerCase()
  if (mime === 'application/json') return 'Json'
  if (mime === 'text/plain') return 'Txt'
  if (mime === 'text/csv') return 'Csv'
  if (mime === 'application/vnd.openxmlformats-officedocument.wordprocessingml.document') {
    return 'Docx'
  }
  if (mime === 'application/pdf') return 'Pdf'
  return null
}

function toBase64(arrayBuffer: ArrayBuffer): string {
  const bytes = new Uint8Array(arrayBuffer)
  const chunkSize = 0x8000
  let binary = ''

  for (let index = 0; index < bytes.length; index += chunkSize) {
    const chunk = bytes.subarray(index, index + chunkSize)
    binary += String.fromCharCode(...chunk)
  }

  return btoa(binary)
}

export async function readUploadPayload(
  selectedFile: File,
  format: ApiScenarioDocumentFormat,
): Promise<{ document_content: string; document_content_base64?: string }> {
  logIntakeEvent('payload_prepare_started', {
    file_name: selectedFile.name,
    format,
    file_size_bytes: selectedFile.size,
  })

  if (format === 'Docx' || format === 'Pdf') {
    const bytes = await selectedFile.arrayBuffer()
    const payload = {
      document_content: '',
      document_content_base64: toBase64(bytes),
    }
    logIntakeEvent('payload_prepare_completed', {
      file_name: selectedFile.name,
      format,
      binary_bytes: bytes.byteLength,
      base64_length: payload.document_content_base64.length,
      text_length: payload.document_content.length,
    })
    return payload
  }

  const text = await selectedFile.text()
  logIntakeEvent('payload_prepare_completed', {
    file_name: selectedFile.name,
    format,
    text_length: text.length,
    base64_length: 0,
  })
  return { document_content: text }
}

export function formatCurrency(value: number | undefined): string {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return '-'
  }
  return value.toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  })
}
