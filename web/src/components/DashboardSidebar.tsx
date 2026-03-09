import WorkflowTabs from './WorkflowTabs'
import type { Workflow } from '../types/api'

type DashboardSidebarProps = {
  workflow: Workflow
  onWorkflowChange: (workflow: Workflow) => void
}

function DashboardSidebar({
  workflow,
  onWorkflowChange,
}: DashboardSidebarProps) {
  return (
    <aside className="dashboard-sidebar">
      <div className="sidebar-panel sidebar-panel-brand">
        <div className="brand-block">
          <div>
            <p className="brand-eyebrow">Estate</p>
            <h1>Optimization Dashboard</h1>
          </div>
        </div>
      </div>

      <div className="sidebar-panel sidebar-panel-nav">
        <p className="sidebar-section-label">Navigation Matrix</p>
        <WorkflowTabs workflow={workflow} onChange={onWorkflowChange} />
      </div>
    </aside>
  )
}

export default DashboardSidebar
