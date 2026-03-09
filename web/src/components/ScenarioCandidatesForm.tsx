import ScenarioDraftEditor from './ScenarioDraftEditor'
import type {
  ScenarioBuilderAssetDraft,
  ScenarioBuilderDraft,
} from '../types/scenarioBuilder'

type ScenarioCandidatesFormProps = {
  drafts: ScenarioBuilderDraft[]
  isSubmitting: boolean
  onDraftFieldChange: <K extends keyof ScenarioBuilderDraft>(
    index: number,
    field: K,
    value: ScenarioBuilderDraft[K],
  ) => void
  onAssetFieldChange: (
    index: number,
    assetId: string,
    field: keyof ScenarioBuilderAssetDraft,
    value: string | boolean,
  ) => void
  onAddAsset: (index: number) => void
  onRemoveAsset: (index: number, assetId: string) => void
  onAddScenario: () => void
  onRemoveScenario: (index: number) => void
  onSubmit: () => void
}

function ScenarioCandidatesForm({
  drafts,
  isSubmitting,
  onDraftFieldChange,
  onAssetFieldChange,
  onAddAsset,
  onRemoveAsset,
  onAddScenario,
  onRemoveScenario,
  onSubmit,
}: ScenarioCandidatesFormProps) {
  return (
    <div className="document-workflow-form">
      <section className="document-console">
        <div className="document-console-header">
          <div className="document-console-heading">
            <p className="document-console-label">Phase 01</p>
            <h3>Stage candidate scenarios</h3>
            <p>
              Build alternative estate plans side by side. The optimizer scores each
              candidate and selects the strongest path.
            </p>
          </div>
          <div className="document-console-state has-file">
            <span>Optimizer inputs</span>
            <strong>{drafts.length} candidate scenarios loaded</strong>
          </div>
        </div>

        <div className="scenario-candidate-list">
          {drafts.map((draft, index) => (
            <div className="candidate-shell" key={`${draft.label}-${index}`}>
              <div className="section-header">
                <div className="scenario-candidate-heading">
                  <p className="mode-card-code">Candidate {index + 1}</p>
                  <h3>{draft.label}</h3>
                </div>
                {drafts.length > 2 && (
                  <button className="btn btn-secondary" type="button" onClick={() => onRemoveScenario(index)}>
                    Remove Candidate
                  </button>
                )}
              </div>
              <ScenarioDraftEditor
                draft={draft}
                onDraftChange={(field, value) => onDraftFieldChange(index, field, value)}
                onAssetChange={(assetId, field, value) => onAssetFieldChange(index, assetId, field, value)}
                onAddAsset={() => onAddAsset(index)}
                onRemoveAsset={(assetId) => onRemoveAsset(index, assetId)}
                canRemoveAsset={draft.assets.length > 1}
              />
            </div>
          ))}
        </div>
      </section>

      <section className="document-console document-console-footer">
        <div className="workflow-action-bar">
          <div className="workflow-action-note">
            <p className="document-console-label">Phase 02</p>
            <h3>Run optimizer</h3>
            <p className="field-help">
              Add more candidates when you need broader comparisons, then submit the
              workspace to rank and select the best scenario.
            </p>
          </div>
          <div className="actions">
            <button className="btn btn-secondary" type="button" onClick={onAddScenario}>
              Add Candidate
            </button>
            <button className="btn btn-primary" type="button" onClick={onSubmit} disabled={isSubmitting}>
              {isSubmitting ? 'Optimizing...' : 'Optimize Scenarios'}
            </button>
          </div>
        </div>
      </section>
    </div>
  )
}

export default ScenarioCandidatesForm
