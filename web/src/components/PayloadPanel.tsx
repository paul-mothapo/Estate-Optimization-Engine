type PayloadPanelProps = {
  payloadPreview: string
}

function PayloadPanel({ payloadPreview }: PayloadPanelProps) {
  return (
    <section className="panel technical-panel">
      <div className="technical-panel-header">
        <p>Technical Payload</p>
        <span>Workspace Trace</span>
      </div>
      <details>
        <summary>Open payload stream</summary>
        <pre>{payloadPreview}</pre>
      </details>
    </section>
  )
}

export default PayloadPanel
