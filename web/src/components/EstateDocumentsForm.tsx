import type { ChangeEvent } from 'react'

type EstateDocumentsFormProps = {
  inputVersion: number
  files: File[]
  isSubmitting: boolean
  onChange: (event: ChangeEvent<HTMLInputElement>) => void
  onSubmit: () => void
}

function EstateDocumentsForm({
  inputVersion,
  files,
  isSubmitting,
  onChange,
  onSubmit,
}: EstateDocumentsFormProps) {
  return (
    <>
      <label className="field-label" htmlFor="estate-file">
        Estate Legal/Tax Documents
      </label>
      <div className="upload-box">
        <input
          key={inputVersion}
          id="estate-file"
          type="file"
          multiple
          accept=".pdf,.docx,.csv,.txt,.json,application/pdf,application/vnd.openxmlformats-officedocument.wordprocessingml.document,text/csv,text/plain,application/json"
          onChange={onChange}
        />
      </div>
      <p className="field-help">
        Upload multiple legal and tax documents for estate-pack completeness analysis.
      </p>
      <div className="file-chip-row">
        {files.map((file) => (
          <span key={`${file.name}-${file.size}`} className="file-chip">
            {file.name}
          </span>
        ))}
      </div>
      <div className="actions">
        <button className="btn btn-primary" type="button" onClick={onSubmit} disabled={isSubmitting}>
          {isSubmitting ? 'Analyzing...' : 'Analyze Estate Pack'}
        </button>
      </div>
    </>
  )
}

export default EstateDocumentsForm
