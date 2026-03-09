type HealthChecksFormProps = {
  isSubmitting: boolean
  onApiHealth: () => void
  onDatabaseHealth: () => void
}

function HealthChecksForm({
  isSubmitting,
  onApiHealth,
  onDatabaseHealth,
}: HealthChecksFormProps) {
  return (
    <>
      <p className="field-help">
        Verify API liveness and database readiness directly from the UI.
      </p>
      <div className="actions">
        <button className="btn btn-primary" type="button" onClick={onApiHealth} disabled={isSubmitting}>
          Check API Health
        </button>
        <button className="btn btn-secondary" type="button" onClick={onDatabaseHealth} disabled={isSubmitting}>
          Check Database Health
        </button>
      </div>
    </>
  )
}

export default HealthChecksForm
