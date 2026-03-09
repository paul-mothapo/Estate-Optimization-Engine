type StatusMessageProps = {
  className: string
  text: string
}

function StatusMessage({ className, text }: StatusMessageProps) {
  return <p className={className}>{text}</p>
}

export default StatusMessage
