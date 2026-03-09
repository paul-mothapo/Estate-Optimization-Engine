import {
  FileSearch,
  Files,
  HeartPulse,
  Scale,
  ShieldCheck,
  Sparkles,
} from 'lucide-react'
import type { Workflow } from '../types/api'

type WorkflowTabsProps = {
  workflow: Workflow
  onChange: (workflow: Workflow) => void
}

const WORKFLOW_OPTIONS: Array<{
  value: Workflow
  label: string
  code: string
  icon: typeof FileSearch
}> = [
  { value: 'document', label: 'Document Intake', code: 'DOC', icon: FileSearch },
  { value: 'scenario', label: 'Scenario Planning', code: 'SCN', icon: Scale },
  { value: 'optimize', label: 'Strategy Optimizer', code: 'OPT', icon: Sparkles },
  { value: 'estate-docs', label: 'Estate Legal Intake', code: 'EST', icon: Files },
  { value: 'rules', label: 'Rules Explorer', code: 'RUL', icon: ShieldCheck },
  { value: 'health', label: 'Service Health', code: 'OPS', icon: HeartPulse },
]

function WorkflowTabs({ workflow, onChange }: WorkflowTabsProps) {
  return (
    <nav className="workflow-nav" aria-label="Primary workflow navigation">
      <div className="workflow-grid">
        {WORKFLOW_OPTIONS.map((option) => {
          const Icon = option.icon
          const isActive = workflow === option.value

          return (
            <button
              key={option.value}
              className={`chip ${isActive ? 'chip-active' : ''}`}
              type="button"
              onClick={() => onChange(option.value)}
            >
              <span className="chip-icon">
                <Icon size={16} strokeWidth={2} />
              </span>
              <span className="chip-copy">
                <strong>{option.label}</strong>
                <span>{option.code}</span>
              </span>
            </button>
          )
        })}
      </div>
    </nav>
  )
}

export default WorkflowTabs
