import type { ChangeEvent } from 'react'
import type { RunMode } from '../types/api'

const supportedFormats = ['JSON', 'CSV', 'TXT', 'DOCX', 'PDF']

type DocumentWorkflowFormProps = {
  inputVersion: number
  file: File | null
  mode: RunMode
  isSubmitting: boolean
  onModeChange: (mode: RunMode) => void
  onFileChange: (event: ChangeEvent<HTMLInputElement>) => void
  onProcess: () => void
  onClear: () => void
}

function DocumentWorkflowForm({
  inputVersion,
  file,
  mode,
  isSubmitting,
  onModeChange,
  onFileChange,
  onProcess,
  onClear,
}: DocumentWorkflowFormProps) {
  return (
    <div className="document-workflow-form">
      <section className="document-console">
        <div className="document-console-header">
          <div className="document-console-heading">
            <p className="document-console-label">Phase 01</p>
            <h3>Upload source document</h3>
            <p>
              Start with one scenario file. Estate legal and tax forms are routed to
              intake analysis automatically.
            </p>
          </div>
          <div className={`document-console-state${file ? ' has-file' : ''}`}>
            <span>{file ? 'Queued for run' : 'Awaiting upload'}</span>
            <strong>{file ? file.name : 'No document selected'}</strong>
          </div>
        </div>

        <div className="document-upload-layout">
          <div className="document-console-panel document-upload-main">
            <label className="field-label" htmlFor="scenario-file">
              Scenario Document
            </label>
            <div className="upload-box">
              <input
                key={inputVersion}
                id="scenario-file"
                type="file"
                accept=".pdf,.docx,.csv,.txt,.json,application/pdf,application/vnd.openxmlformats-officedocument.wordprocessingml.document,text/csv,text/plain,application/json"
                onChange={onFileChange}
              />
            </div>
            <p className="field-help">
              Use JSON directly, CSV rows with <code>scenario_json</code>, or
              TXT/DOCX/PDF content with embedded JSON or <code>key: value</code>{' '}
              fields.
            </p>
          </div>

          <aside className="document-console-panel document-upload-summary" aria-label="Document upload summary">
            <span className="upload-summary-label">Accepted formats</span>
            <div className="file-chip-row">
              {supportedFormats.map((format) => (
                <span key={format} className="file-chip">
                  {format}
                </span>
              ))}
            </div>
            <div className={`document-selected-state${file ? ' has-file' : ''}`}>
              <span>{file ? 'Ready to process' : 'Waiting for upload'}</span>
              <strong>{file ? file.name : 'Choose one document to continue.'}</strong>
            </div>
            <div className="document-selected-meta">
              <div className="document-selected-meta-row">
                <span>Current profile</span>
                <strong>{mode === 'calculate' ? 'Parse, validate, calculate' : 'Parse and validate'}</strong>
              </div>
              <div className="document-selected-meta-row">
                <span>Input policy</span>
                <strong>Single source document per run</strong>
              </div>
            </div>
          </aside>
        </div>
      </section>

      <section className="document-console">
        <div className="document-console-header">
          <div className="document-console-heading">
            <p className="document-console-label">Phase 02</p>
            <h3>Choose processing depth</h3>
            <p>Decide whether this run should stop at validation or continue into calculation.</p>
          </div>
        </div>

        <fieldset className="mode-group document-mode-group">
          <legend>Run Mode</legend>
          <div className="document-mode-grid">
            <label className={`mode-option mode-card${mode === 'ingest' ? ' mode-option-active' : ''}`}>
              <input
                type="radio"
                name="run-mode"
                value="ingest"
                checked={mode === 'ingest'}
                onChange={() => onModeChange('ingest')}
              />
              <div>
                <p className="mode-card-code">PROFILE.A</p>
                <strong>Parse + validate</strong>
                <span>Inspect extracted scenarios and confirm the document structure before calculation.</span>
              </div>
            </label>
            <label className={`mode-option mode-card${mode === 'calculate' ? ' mode-option-active' : ''}`}>
              <input
                type="radio"
                name="run-mode"
                value="calculate"
                checked={mode === 'calculate'}
                onChange={() => onModeChange('calculate')}
              />
              <div>
                <p className="mode-card-code">PROFILE.B</p>
                <strong>Parse + validate + calculate</strong>
                <span>Run the full path immediately after validation and return scenario outputs.</span>
              </div>
            </label>
          </div>
        </fieldset>
      </section>

      <section className="document-console document-console-footer">
        <div className="workflow-action-bar">
          <div className="workflow-action-note">
            <p className="document-console-label">Phase 03</p>
            <h3>Submit workspace run</h3>
            <p className="field-help">
              {mode === 'calculate'
                ? 'The engine will validate the document and immediately produce results.'
                : 'Use validation-only when you want to verify extracted fields before calculation.'}
            </p>
          </div>
          <div className="actions">
            <button
              className="btn btn-primary"
              type="button"
              onClick={onProcess}
              disabled={isSubmitting || !file}
            >
              {isSubmitting ? 'Processing...' : 'Process Document'}
            </button>
            <button className="btn btn-secondary" type="button" onClick={onClear}>
              Reset Output
            </button>
          </div>
        </div>
      </section>
    </div>
  )
}

export default DocumentWorkflowForm
