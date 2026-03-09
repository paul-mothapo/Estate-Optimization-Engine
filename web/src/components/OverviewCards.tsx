type OverviewCardsProps = {
  cards: Array<{
    label: string
    value: string
    helper: string
  }>
}

function OverviewCards({ cards }: OverviewCardsProps) {
  return (
    <section className="overview-grid">
      {cards.map((card) => (
        <article className="overview-card panel" key={card.label}>
          <div className="overview-card-head">
            <p className="overview-label">{card.label}</p>
          </div>
          <strong>{card.value}</strong>
          <span>{card.helper}</span>
        </article>
      ))}
    </section>
  )
}

export default OverviewCards
