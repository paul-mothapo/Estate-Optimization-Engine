type IntakeLogDetails = Record<string, unknown>

function intakeLogsEnabled(): boolean {
  if (typeof window === 'undefined') {
    return false
  }

  if (window.localStorage.getItem('engine:intake-logs') === '1') {
    return true
  }

  return import.meta.env.DEV
}

function withTimestamp(details: IntakeLogDetails): IntakeLogDetails {
  return {
    ...details,
    at: new Date().toISOString(),
  }
}

export function logIntakeEvent(event: string, details: IntakeLogDetails = {}): void {
  if (!intakeLogsEnabled()) {
    return
  }

  console.info(`[engine:intake] ${event}`, withTimestamp(details))
}

export function logIntakeError(event: string, details: IntakeLogDetails = {}): void {
  if (!intakeLogsEnabled()) {
    return
  }

  console.error(`[engine:intake] ${event}`, withTimestamp(details))
}
