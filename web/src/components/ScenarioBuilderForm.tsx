import ScenarioDraftEditor from './ScenarioDraftEditor'
import type {
  ScenarioBuilderAssetDraft,
  ScenarioBuilderDraft,
} from '../types/scenarioBuilder'

type ScenarioBuilderFormProps = {
  draft: ScenarioBuilderDraft
  isSubmitting: boolean
  onDraftChange: <K extends keyof ScenarioBuilderDraft>(
    field: K,
    value: ScenarioBuilderDraft[K],
  ) => void
  onAssetChange: (
    assetId: string,
    field: keyof ScenarioBuilderAssetDraft,
    value: string | boolean,
  ) => void
  onAddAsset: () => void
  onRemoveAsset: (assetId: string) => void
  onSubmit: () => void
}

function ScenarioBuilderForm({
  draft,
  isSubmitting,
  onDraftChange,
  onAssetChange,
  onAddAsset,
  onRemoveAsset,
  onSubmit,
}: ScenarioBuilderFormProps) {
  return (
    <>
      <p className="field-help">
        Capture estate facts in plain fields. The engine payload is generated internally.
      </p>
      <ScenarioDraftEditor
        draft={draft}
        onDraftChange={onDraftChange}
        onAssetChange={onAssetChange}
        onAddAsset={onAddAsset}
        onRemoveAsset={onRemoveAsset}
        canRemoveAsset={draft.assets.length > 1}
      />
      <div className="actions">
        <button className="btn btn-primary" type="button" onClick={onSubmit} disabled={isSubmitting}>
          {isSubmitting ? 'Calculating...' : 'Calculate Scenario'}
        </button>
      </div>
    </>
  )
}

export default ScenarioBuilderForm
