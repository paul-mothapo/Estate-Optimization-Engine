import {
  Activity,
  AlertTriangle,
  FileSearch,
  Files,
  HeartPulse,
  RefreshCw,
  Scale,
  ShieldCheck,
  CheckCircle2,
  Sparkles,
} from 'lucide-react'
import type { StatusTone, Workflow } from '../types/api'

type DashboardTopBarProps = {
  workflow: Workflow
  statusTone: StatusTone
  statusText: string
  onReset: () => void
  isSubmitting: boolean
}

const WORKFLOW_META: Record<
  Workflow,
  { title: string; eyebrow: string; code: string; icon: typeof FileSearch }
> = {
  document: {
    title: 'Document Intake',
    eyebrow: 'Scenario Documents',
    code: 'DOC.INTAKE',
    icon: FileSearch,
  },
  scenario: {
    title: 'Scenario Planning',
    eyebrow: 'Direct Modeling',
    code: 'SCN.PLAN',
    icon: Scale,
  },
  optimize: {
    title: 'Strategy Optimizer',
    eyebrow: 'Candidate Comparison',
    code: 'OPT.SELECT',
    icon: Sparkles,
  },
  'estate-docs': {
    title: 'Estate Legal Intake',
    eyebrow: 'Document Readiness',
    code: 'EST.READY',
    icon: Files,
  },
  rules: {
    title: 'Rules Explorer',
    eyebrow: 'Reference Data',
    code: 'RULES.REG',
    icon: ShieldCheck,
  },
  health: {
    title: 'Service Health',
    eyebrow: 'Platform Checks',
    code: 'OPS.HEALTH',
    icon: HeartPulse,
  },
}

const STATUS_META: Record<StatusTone, { label: string; icon: typeof Activity }> = {
  idle: { label: 'Idle', icon: Activity },
  busy: { label: 'Running', icon: RefreshCw },
  success: { label: 'Ready', icon: CheckCircle2 },
  error: { label: 'Attention', icon: AlertTriangle },
}

function DashboardTopBar({
  workflow,
  statusTone,
  statusText,
  onReset,
  isSubmitting,
}: DashboardTopBarProps) {
  const workflowMeta = WORKFLOW_META[workflow]
  const statusMeta = STATUS_META[statusTone]
  const WorkflowGlyph = workflowMeta.icon
  const StatusGlyph = statusMeta.icon

  return (
    <header className="dashboard-topbar panel">
      <div className="topbar-main">
        <div className="topbar-mark" aria-hidden="true">
          <WorkflowGlyph size={24} strokeWidth={2.1} />
        </div>

        <div className="topbar-copy">
          <div className="topbar-kicker-row">
            <p className="topbar-kicker">{workflowMeta.eyebrow}</p>
            <span className="topbar-code">{workflowMeta.code}</span>
          </div>
          <div className="topbar-title-row">
            <h2>{workflowMeta.title}</h2>
            <span className="topbar-divider" aria-hidden="true" />
            <span className={`status-pill status-pill-${statusTone} topbar-status-pill`}>
              <StatusGlyph size={14} strokeWidth={2} />
              {statusMeta.label}
            </span>
          </div>
          <p className="topbar-subtitle">{statusText}</p>
        </div>
      </div>

      <div className="topbar-actions">
        <button className="btn btn-secondary topbar-reset" type="button" onClick={onReset} disabled={isSubmitting}>
          <RefreshCw size={16} strokeWidth={2} />
          Reset Workspace
        </button>
      </div>
    </header>
  )
}

export default DashboardTopBar
