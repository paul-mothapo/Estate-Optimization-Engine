import type { Workflow } from '../types/api'

const DEFAULT_WORKFLOW: Workflow = 'document'

const WORKFLOW_PATHS: Record<Workflow, string> = {
  document: '/web/document-intake',
  scenario: '/web/scenario-planning',
  optimize: '/web/strategy-optimizer',
  'estate-docs': '/web/estate-legal-intake',
  rules: '/web/rules-explorer',
  health: '/web/service-health',
}

function normalizePathname(pathname: string): string {
  const trimmed = pathname.replace(/\/+$/, '')
  return trimmed.length > 0 ? trimmed : '/'
}

export function getWorkflowPath(workflow: Workflow): string {
  return WORKFLOW_PATHS[workflow]
}

export function getWorkflowFromPath(pathname: string): Workflow {
  const normalizedPathname = normalizePathname(pathname)

  if (normalizedPathname === '/web') {
    return DEFAULT_WORKFLOW
  }

  for (const [workflow, path] of Object.entries(WORKFLOW_PATHS) as Array<[Workflow, string]>) {
    if (normalizePathname(path) === normalizedPathname) {
      return workflow
    }
  }

  return DEFAULT_WORKFLOW
}

