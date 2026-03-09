type HeroSectionProps = {
  kicker: string
  title: string
  description: string
}

function HeroSection({ kicker, title, description }: HeroSectionProps) {
  return (
    <header className="hero panel">
      <div className="hero-shell">
        <p className="hero-kicker">{kicker}</p>
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </header>
  )
}

export default HeroSection
