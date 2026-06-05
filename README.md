# ternary-sandpile

**Drop grains, watch avalanches, find the power law. The universe's favorite distribution, in three states.**

The Abelian sandpile model is the canonical example of self-organized criticality (SOC). Start with a flat grid. Drop grains of sand one at a time at random positions. When a cell reaches height 4, it *topples* — giving one grain to each of its 4 neighbors. Those neighbors might also topple. Cascades — *avalanches* — propagate across the grid.

The remarkable property: without any parameter tuning, the system *self-organizes* to a critical state where avalanche sizes follow a power law. Small avalanches are common. Huge avalanches are rare but inevitable. This is the same distribution as earthquakes, stock market crashes, neural avalanches, and extinction events. The universe keeps choosing this distribution. The sandpile shows why.

## What's Inside

- **`Sandpile`** — the grid of u8 heights. `drop(x, y)` adds one grain
- **`topple()`** — one round of toppling. Returns avalanche size
- **`stabilize()`** — topple repeatedly until stable. Returns total avalanche size
- **`is_stable()`** — are any cells at or above critical height?
- **`height_at(x, y)`** — read current height
- **`total_grains()`** — conservation check (grains are never created or destroyed)
- **`avalanche_series(drops)`** — drop N grains, record each avalanche size
- **`toppling_histogram()`** — which cells topple most? The "hot spots"

## Quick Example

```rust
use ternary_sandpile::*;

let mut pile = Sandpile::new(20, 20);

// Drop 1000 grains at the center
let mut sizes = Vec::new();
for _ in 0..1000 {
    pile.drop(10, 10);
    let avalanche = pile.stabilize();
    sizes.push(avalanche);
}

// The distribution follows a power law
let max = sizes.iter().max().unwrap();
let min = sizes.iter().filter(|&&a| a > 0).min().unwrap();
println!("Avalanches from {} to {} cells", min, max);

// Where are the hot spots?
let hist = pile.toppling_histogram();
// Center topples most — the epicenter of chaos
```

## The Deeper Truth

**Criticality is free.** You don't tune a temperature parameter (unlike Ising). You don't adjust coupling (unlike Kuramoto). You just drop grains and the system *finds* criticality on its own. This is why SOC matters — it explains how complex, scale-invariant behavior emerges from simple local rules without external control.

The power law is the signature. Plot log(size) vs log(frequency): straight line. That line is the fingerprint of self-organized criticality — and it's the same fingerprint found in neural avalanches (measured in cortical tissue), solar flares, and the Gutenberg-Richter earthquake law. The sandpile isn't just a model. It's a lens for seeing the hidden order in apparently random catastrophes.

The ternary mapping: cells below critical height are "stable" (0 — quiet). Cells at height ≥4 are "critical" (+1 — about to topple, propagating change). Recently toppled cells are "depleted" (-1). The boundary between stable and critical is where all the action is — the *edge of chaos*.

**Use cases:**
- **Complex systems research** — the canonical SOC model
- **Neuroscience** — neural avalanche dynamics follow the same power law
- **Seismology** — earthquake statistics from simple rules
- **Financial modeling** — market crashes as SOC avalanches
- **Art** — sandpile toppling patterns are visually stunning
- **Education** — the most accessible example of emergence

## See Also

- **ternary-fire** — fire is SOC with a biological clock (growth → burn → regrow)
- **ternary-ising** — phase transitions (SOC with a temperature dial)
- **ternary-percolation** — spatial connectivity (critical thresholds without dynamics)
- **ternary-life** — complex dynamics without criticality
- **ternary-irradiate** — cascade dynamics with inverse-square propagation
- **ternary-visualizer** — render sandpile patterns as ASCII art

## Install

```bash
cargo add ternary-sandpile
```

## License

MIT
