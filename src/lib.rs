#![forbid(unsafe_code)]

/// Self-organized criticality on ternary grids.
/// Heights are u8; a cell topples when height >= 4 (gives 1 to each of 4 neighbors).
pub struct Sandpile {
    heights: Vec<u8>,
    width: usize,
}

const CRITICAL: u8 = 4;

impl Sandpile {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            heights: vec![0u8; width * height],
            width,
        }
    }

    fn h(&self) -> usize {
        if self.width == 0 { 0 } else { self.heights.len() / self.width }
    }

    /// Drop one grain at (x, y)
    pub fn drop(&mut self, x: usize, y: usize) {
        let w = self.width;
        let h = self.h();
        if x < w && y < h {
            self.heights[y * w + x] = self.heights[y * w + x].saturating_add(1);
        }
    }

    /// Topple all unstable cells once. Returns total avalanche size (number of topples).
    pub fn topple(&mut self) -> usize {
        let w = self.width;
        let h = self.h();
        let mut avalanche = 0usize;

        // Find cells to topple
        let mut topples: Vec<(usize, usize)> = Vec::new();
        for y in 0..h {
            for x in 0..w {
                if self.heights[y * w + x] >= CRITICAL {
                    topples.push((x, y));
                }
            }
        }

        for (x, y) in topples {
            let i = y * w + x;
            if self.heights[i] >= CRITICAL {
                self.heights[i] -= CRITICAL;
                let dirs: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                for (dx, dy) in dirs {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 {
                        let ux = nx as usize;
                        let uy = ny as usize;
                        if ux < w && uy < h {
                            self.heights[uy * w + ux] += 1;
                        }
                    }
                }
                avalanche += 1;
            }
        }
        avalanche
    }

    /// Check if all cells are below critical height
    pub fn is_stable(&self) -> bool {
        self.heights.iter().all(|&h| h < CRITICAL)
    }

    /// Perform multiple drops at center, returning avalanche sizes
    pub fn avalanche_history(&mut self, drops: usize) -> Vec<usize> {
        let cx = self.width / 2;
        let cy = self.h() / 2;
        let mut history = Vec::new();
        for _ in 0..drops {
            self.drop(cx, cy);
            let mut total = 0;
            loop {
                let a = self.topple();
                if a == 0 { break; }
                total += a;
            }
            history.push(total);
        }
        history
    }

    /// The critical height threshold
    pub fn critical_height(&self) -> u8 {
        CRITICAL
    }

    pub fn height_at(&self, x: usize, y: usize) -> u8 {
        let w = self.width;
        let h = self.h();
        if x < w && y < h {
            self.heights[y * w + x]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sandpile() {
        let s = Sandpile::new(5, 5);
        assert!(s.is_stable());
        assert_eq!(s.height_at(2, 2), 0);
    }

    #[test]
    fn test_drop_grain() {
        let mut s = Sandpile::new(3, 3);
        s.drop(1, 1);
        assert_eq!(s.height_at(1, 1), 1);
    }

    #[test]
    fn test_no_topple_below_critical() {
        let mut s = Sandpile::new(3, 3);
        s.drop(1, 1);
        s.drop(1, 1);
        s.drop(1, 1);
        assert_eq!(s.topple(), 0);
        assert_eq!(s.height_at(1, 1), 3);
    }

    #[test]
    fn test_topple_at_critical() {
        let mut s = Sandpile::new(3, 3);
        for _ in 0..4 { s.drop(1, 1); }
        assert!(!s.is_stable());
        let a = s.topple();
        assert_eq!(a, 1);
        assert_eq!(s.height_at(1, 1), 0);
        assert_eq!(s.height_at(0, 1), 1);
        assert_eq!(s.height_at(2, 1), 1);
        assert_eq!(s.height_at(1, 0), 1);
        assert_eq!(s.height_at(1, 2), 1);
    }

    #[test]
    fn test_topple_corner() {
        let mut s = Sandpile::new(3, 3);
        for _ in 0..4 { s.drop(0, 0); }
        let a = s.topple();
        assert_eq!(a, 1);
        assert_eq!(s.height_at(0, 0), 0);
        assert_eq!(s.height_at(1, 0), 1);
        assert_eq!(s.height_at(0, 1), 1);
    }

    #[test]
    fn test_avalanche_history() {
        let mut s = Sandpile::new(5, 5);
        let hist = s.avalanche_history(5);
        assert_eq!(hist.len(), 5);
        // First 3 drops: no avalanche (height 1,2,3)
        assert_eq!(hist[0], 0);
        assert_eq!(hist[1], 0);
        assert_eq!(hist[2], 0);
        assert_eq!(hist[3], 1); // 4th drop reaches critical
    }

    #[test]
    fn test_is_stable_after_topple() {
        let mut s = Sandpile::new(5, 5);
        for _ in 0..4 { s.drop(2, 2); }
        assert!(!s.is_stable());
        while s.topple() > 0 {}
        assert!(s.is_stable());
    }

    #[test]
    fn test_critical_height_is_4() {
        let s = Sandpile::new(3, 3);
        assert_eq!(s.critical_height(), 4);
    }

    #[test]
    fn test_cascade_topple() {
        let mut s = Sandpile::new(3, 3);
        // Build up center to 4 and neighbor to 3
        for _ in 0..4 { s.drop(1, 1); }
        for _ in 0..3 { s.drop(0, 1); }
        // Topple center: gives 1 to (0,1) making it 4
        let a = s.topple();
        assert_eq!(a, 1);
        assert_eq!(s.height_at(0, 1), 4);
    }

    #[test]
    fn test_avalanche_large() {
        let mut s = Sandpile::new(7, 7);
        let hist = s.avalanche_history(15);
        let total: usize = hist.iter().sum();
        assert!(total > 0);
    }

    #[test]
    fn test_drop_out_of_bounds() {
        let mut s = Sandpile::new(3, 3);
        s.drop(5, 5);
        assert!(s.is_stable());
    }

    #[test]
    fn test_multiple_topple_rounds() {
        let mut s = Sandpile::new(3, 3);
        for _ in 0..12 { s.drop(1, 1); }
        let mut rounds = 0;
        while s.topple() > 0 { rounds += 1; }
        assert!(rounds >= 1);
        assert!(s.is_stable());
    }

    #[test]
    fn test_single_cell() {
        let mut s = Sandpile::new(1, 1);
        for _ in 0..4 { s.drop(0, 0); }
        // Single cell has no neighbors, grains are lost when toppled
        let a = s.topple();
        assert_eq!(a, 1);
        assert_eq!(s.height_at(0, 0), 0);
    }

    #[test]
    fn test_conservation() {
        let mut s = Sandpile::new(9, 9);
        s.avalanche_history(20);
        let mut total = 0usize;
        for y in 0..9 {
            for x in 0..9 {
                total += s.height_at(x, y) as usize;
            }
        }
        // Some grains lost at edges, but most should remain
        assert!(total > 15);
    }
}
