# ternary-sandpile

**Self-organized criticality. Drop grains, watch avalanches, find the power law.**

The Abelian sandpile model is the canonical example of self-organized criticality (SOC). Start with a flat grid. Drop grains of sand one at a time. When a cell reaches height 4, it *topples* — giving one grain to each of its 4 neighbors. Those neighbors might also topple. Cascades — *avalanches* — propagate across the grid.

The remarkable property: without any parameter tuning, the system *self-organizes* to a critical state where avalanche sizes follow a power law. Small avalanches are common. Huge avalanches are rare but inevitable. This is the same distribution as earthquakes, stock market crashes, and extinction events.

This crate implements the sandpile model with avalanche tracking, critical state analysis, and toppling statistics.

## What's Inside

- **`Sandpile`** — the grid of heights. `drop(x, y)` adds one grain
- **`topple()`** — one round of toppling. Returns avalanche size (number of cells that toppled)
- **`stabilize()`** — topple repeatedly until stable. Returns total avalanche size
- **`is_stable()`** — are any cells at or above the critical height?
- **`height_at(x, y)`** — read the current height
- **`total_grains()`** — sum of all heights (conservation check)
- **`avalanche_series(drops)`** — drop N grains, record each avalanche size. The power law lives here
- **`toppling_histogram()`** — how often does each cell topple? The "hot spots"

## Quick Example

```rust
use ternary_sandpile::*;

let mut pile = Sandpile::new(20, 20);

// Drop 1000 grains at the center, one at a time
let mut avalanche_sizes = Vec::new();
for _ in 0..1000 {
    pile.drop(10, 10);
    let avalanche = pile.stabilize();
    avalanche_sizes.push(avalanche);
}

// Avalanche size distribution should follow a power law
let max = *avalanche_sizes.iter().max().unwrap_or(&0);
let min = *avalanche_sizes.iter().filter(|&&a| a > 0).min().unwrap_or(&1);
println!("Avalanche sizes: {} to {}", min, max);

// The toppling histogram shows which cells topple most
let histogram = pile.toppling_histogram();
// Center area topples most, edges are stable
```

## The Deeper Truth

**Criticality is free.** You don't need to tune a temperature parameter (unlike the Ising model). You don't need to adjust a coupling constant (unlike Kuramoto). You just drop grains and the system *finds* the critical state on its own. This is why SOC is so important in physics — it explains how complex, scale-invariant behavior emerges from simple local rules without any external control.

The sandpile connects to ternary systems through the critical height: a cell at height 0, 1, 2, or 3 is "stable" (analogous to ternary 0 — not doing anything interesting). A cell at height ≥4 is "critical" (analogous to +1 — actively propagating change). After toppling, the cell returns to stability (back to 0). The ternary mapping: height < 4 → 0, height = 4 → +1 (about to topple), recently toppled → -1 (depleted).

The power law is the signature. If you plot log(avalanche_size) vs log(frequency), you get a straight line. That line is the fingerprint of self-organized criticality — and it's the same fingerprint found in neural avalanches, solar flares, and Gutenberg-Richter earthquake statistics.

**Use cases:**
- **Complex systems research** — the simplest SOC model
- **Neuroscience** — neural avalanche dynamics follow the same power law
- **Seismology** — earthquake statistics from a simple model
- **Financial modeling** — market crashes as SOC avalanches
- **Education** — the most accessible example of emergence and power laws

## See Also

- **ternary-fire** — forest fire model (another SOC system, different mechanism)
- **ternary-ising** — phase transitions (SOC without the self-organization)
- **ternary-percolation** — spatial connectivity (critical thresholds without dynamics)
- **ternary-life** — cellular automaton dynamics (complexity without criticality)

## Install

```bash
cargo add ternary-sandpile
```

## License

MIT
