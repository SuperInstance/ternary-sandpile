# Ternary Sandpile

Self-organized criticality (SOC) on a 2D grid, implementing the classical Abelian sandpile model with a ternary-aware interface. Grains accumulate until a cell reaches **critical height 4**, then topples — distributing one grain to each of its four cardinal neighbors. This simple rule produces fractal patterns, power-law avalanche distributions, and $1/f$ noise.

## Why It Matters

The Abelian sandpile model (ASM), introduced by Bak, Tang, and Wiesenfeld in 1987, is the canonical example of **self-organized criticality** — a system that naturally evolves to a critical state without external tuning. It appears in:

- **Neural avalanches**: cortical activity follows sandpile-like power laws ($p(s) \sim s^{-\tau}$, $\tau \approx 1.5$)
- **Earthquake dynamics**: the Gutenberg-Richter law emerges from sandpile-like stress redistribution
- **Financial markets**: crash cascades propagate like sandpile avalanches
- **GPU fleet load balancing**: when work items pile up on overloaded nodes, redistribution cascades follow the same dynamics

The ternary connection: the SuperInstance ecosystem maps agent states to $\{-1, 0, +1\}$, and the sandpile provides the **background dynamics** — stress accumulation (dropping grains), critical redistribution (toppling), and relaxation (stabilization).

## How It Works

### The Toppling Rule

A cell at position $(x, y)$ with height $h$ **topples** when $h \geq 4$:

$$\text{topple}(x, y): \quad h_{(x,y)} \leftarrow h_{(x,y)} - 4, \quad h_{(x\pm1,y)} \leftarrow h_{(x\pm1,y)} + 1, \quad h_{(x,y\pm1)} \leftarrow h_{(x,y\pm1)} + 1$$

At boundaries, grains that would go outside the grid are **lost** (open boundary conditions).

### Avalanche Dynamics

An **avalanche** is a cascade of topples triggered by a single grain drop. The total avalanche size $a$ is the number of cells that toppled.

Avalanches exhibit a **power-law distribution** at the critical state:

$$P(a > s) \sim s^{-\beta}, \quad \beta \approx 0.25 \text{ in 2D}$$

This means most avalanches are small, but occasionally a massive cascade sweeps the entire grid — the hallmark of criticality.

### Abelian Property

The model is **Abelian**: the final stable state is independent of the order in which cells topple. This means:

- You can process topples in any order
- Parallelization is safe (process all unstable cells simultaneously)
- The `topple()` method does one synchronous round; calling it repeatedly until stable yields the same result as any other order

### Conservation

Grains are conserved except at boundaries. For an $L \times L$ grid with $D$ drops at center:

$$\text{grains retained} = D - \text{boundary loss}$$

For large $L$, boundary loss scales as $O(\sqrt{D})$.

### Complexity

| Operation | Time | Space |
|---|---|---|
| `drop(x, y)` | $O(1)$ | $O(1)$ |
| `topple()` (one round) | $O(W \cdot H)$ | $O(W \cdot H)$ |
| Full stabilization | $O(a)$ where $a$ = avalanche size | $O(1)$ |
| `avalanche_history(drops)` | $O(drops \cdot \bar{a})$ | $O(drops)$ |
| `is_stable()` | $O(W \cdot H)$ | $O(1)$ |

## Quick Start

```rust
use ternary_sandpile::Sandpile;

let mut pile = Sandpile::new(9, 9);

// Drop grains at center and track avalanches
let avalanches = pile.avalanche_history(50);

for (i, &size) in avalanches.iter().enumerate() {
    println!("Drop {}: avalanche size = {}", i + 1, size);
}

// Check final state
println!("Stable: {}", pile.is_stable());
println!("Center height: {}", pile.height_at(4, 4));

// Manual control
let mut p = Sandpile::new(3, 3);
for _ in 0..4 { p.drop(1, 1); }  // Reach critical
assert!(!p.is_stable());
let avalanche = p.topple();       // Returns 1
assert_eq!(p.height_at(1, 1), 0); // Reset to 0
assert_eq!(p.height_at(0, 1), 1); // Neighbor gained 1
```

## API

### `Sandpile`

| Method | Description |
|---|---|
| `new(width, height)` | Create zero-initialized grid |
| `drop(x, y)` | Add one grain at $(x, y)$ |
| `topple() → usize` | One synchronous toppling round; returns avalanche size |
| `is_stable() → bool` | All cells below critical height |
| `avalanche_history(drops) → Vec<usize>` | Drop at center repeatedly, returning each avalanche size |
| `height_at(x, y) → u8` | Query cell height |
| `critical_height() → u8` | Returns 4 (the toppling threshold) |

## Architecture Notes

Within the **γ + η = C** framework:

- **γ (gamma)** — the grain: each unit of "stress" placed on the system (analogous to a ternary agent signaling `+1`)
- **η (eta)** — the toppling rule: the environment's nonlinear response — redistribution cascades that propagate stress across the neighbor graph
- **C** — **criticality**: the system self-organizes to the critical state where it exhibits maximum dynamic range and information processing capacity (the edge of chaos)

The implementation is `#![forbid(unsafe_code)]` with zero external dependencies.

## References

1. Bak, P., Tang, C., & Wiesenfeld, K. (1987). "Self-Organized Criticality: An Explanation of 1/f Noise." *Physical Review Letters*, 59(4), 381. — Original sandpile paper.
2. Dhar, D. (1990). "Self-Organized Critical State of Sandpile Automaton Models." *Physical Review Letters*, 64(14), 1613. — Proof of the Abelian property.
3. Pruessner, G. (2012). *Self-Organised Criticality: Theory, Models and Characterisation*. Cambridge University Press. — Comprehensive textbook on SOC.
4. Beggs, J. M., & Plenz, D. (2003). "Neuronal Avalanches in Neocortical Circuits." *Journal of Neuroscience*, 23(35), 11167-11177. — Power laws in neural activity.

## License

MIT OR Apache-2.0
