pub trait AtlasAllocator {
    fn allocate(&mut self, w: u32, h: u32) -> Option<(u32, u32)>;
    fn dimensions(&self) -> (u32, u32);
}
pub struct ShelfAllocator {
    width: u32,
    height: u32,
    shelves: Vec<Shelf>,
}
struct Shelf {
    y: u32,
    height: u32,
    x_cursor: u32,
}
impl ShelfAllocator {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            shelves: Vec::new(),
        }
    }
}
impl AtlasAllocator for ShelfAllocator {
    fn allocate(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        for shelf in &mut self.shelves {
            if h <= shelf.height && shelf.x_cursor + w <= self.width {
                let x = shelf.x_cursor;
                let y = shelf.y;
                shelf.x_cursor += w;
                return Some((x, y));
            }
        }
        let y = self.shelves.last().map(|s| s.y + s.height).unwrap_or(0);
        if y + h > self.height {
            return None;
        }
        self.shelves.push(Shelf {
            y,
            height: h,
            x_cursor: w,
        });
        Some((0, y))
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
#[derive(Clone, Copy, Debug)]
struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}
pub struct MaxRectsAllocator {
    width: u32,
    height: u32,
    free_rects: Vec<Rect>,
}
impl MaxRectsAllocator {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            free_rects: vec![Rect {
                x: 0,
                y: 0,
                w: width,
                h: height,
            }],
        }
    }
    /// True if rectangles a and b overlap (share any interior area).
    #[inline]
    fn overlaps(a: Rect, b: Rect) -> bool {
        b.x < a.x + a.w && b.x + b.w > a.x && b.y < a.y + a.h && b.y + b.h > a.y
    }
    /// True if a fully contains b.
    #[inline]
    fn contains(a: Rect, b: Rect) -> bool {
        a.x <= b.x && a.y <= b.y && a.x + a.w >= b.x + b.w && a.y + a.h >= b.y + b.h
    }
    /// Append up to four free sub-rects produced by cutting `used` out of `free`.
    /// The four cuts are: left, right, top, bottom strips of `free` around `used`.
    fn push_splits(free: Rect, used: Rect, out: &mut Vec<Rect>) {
        // Left strip
        if used.x > free.x {
            out.push(Rect {
                x: free.x,
                y: free.y,
                w: used.x - free.x,
                h: free.h,
            });
        }
        // Right strip
        if used.x + used.w < free.x + free.w {
            out.push(Rect {
                x: used.x + used.w,
                y: free.y,
                w: (free.x + free.w) - (used.x + used.w),
                h: free.h,
            });
        }
        // Top strip
        if used.y > free.y {
            out.push(Rect {
                x: free.x,
                y: free.y,
                w: free.w,
                h: used.y - free.y,
            });
        }
        // Bottom strip
        if used.y + used.h < free.y + free.h {
            out.push(Rect {
                x: free.x,
                y: used.y + used.h,
                w: free.w,
                h: (free.y + free.h) - (used.y + used.h),
            });
        }
    }
    /// Remove every free rect that is fully contained within another free rect.
    fn prune_free_list(&mut self) {
        let mut i = 0;
        while i < self.free_rects.len() {
            let mut j = i + 1;
            let mut i_removed = false;
            while j < self.free_rects.len() {
                let a = self.free_rects[i];
                let b = self.free_rects[j];
                if Self::contains(a, b) {
                    // b is redundant; fast-remove it, re-check same j position
                    self.free_rects.swap_remove(j);
                } else if Self::contains(b, a) {
                    // a is redundant; remove and stop inner loop
                    self.free_rects.swap_remove(i);
                    i_removed = true;
                    break;
                } else {
                    j += 1;
                }
            }
            if !i_removed {
                i += 1;
            }
        }
    }
    /// One pass of merging adjacent free rects to reduce fragmentation.
    ///
    /// Two rects are mergeable when they share a full edge:
    ///  - Same y and h, and their x-spans are contiguous  →  horizontal merge
    ///  - Same x and w, and their y-spans are contiguous  →  vertical merge
    ///
    /// A single forward pass isn't exhaustive, but is O(n²) and sufficient to
    /// undo the four-cut fragmentation introduced by each placement.
    fn merge_free_rects(&mut self) {
        let mut i = 0;
        while i < self.free_rects.len() {
            let mut j = i + 1;
            while j < self.free_rects.len() {
                let a = self.free_rects[i];
                let b = self.free_rects[j];

                //Horizontal merge (identical y/h, adjacent on x)
                if a.y == b.y && a.h == b.h {
                    if a.x + a.w == b.x {
                        // a is immediately left of b
                        self.free_rects[i].w += b.w;
                        self.free_rects.swap_remove(j);
                        continue; // j now points to swapped-in rect; re-check
                    } else if b.x + b.w == a.x {
                        // b is immediately left of a
                        self.free_rects[i].x = b.x;
                        self.free_rects[i].w += b.w;
                        self.free_rects.swap_remove(j);
                        continue;
                    }
                }
                //Vertical merge (identical x/w, adjacent on y)
                if a.x == b.x && a.w == b.w {
                    if a.y + a.h == b.y {
                        // a is immediately above b
                        self.free_rects[i].h += b.h;
                        self.free_rects.swap_remove(j);
                        continue;
                    } else if b.y + b.h == a.y {
                        // b is immediately above a
                        self.free_rects[i].y = b.y;
                        self.free_rects[i].h += b.h;
                        self.free_rects.swap_remove(j);
                        continue;
                    }
                }
                j += 1;
            }
            i += 1;
        }
    }
}
impl AtlasAllocator for MaxRectsAllocator {
    fn allocate(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        // Best Short Side Fit (BSSF)
        // Primary key:   min(free.w - w, free.h - h)  — shorter leftover side
        // Secondary key: max(free.w - w, free.h - h)  — longer  leftover side
        // Minimising the short side packs rects into the tightest-fitting corner,
        // outperforming Best Area Fit on most real-world sprite-atlas workloads.
        let mut best_score = (u32::MAX, u32::MAX);
        let mut best_idx: Option<usize> = None;
        let mut placed = Rect { x: 0, y: 0, w, h };
        for (i, free) in self.free_rects.iter().enumerate() {
            if w <= free.w && h <= free.h {
                let dw = free.w - w;
                let dh = free.h - h;
                let score = (dw.min(dh), dw.max(dh));
                if score < best_score {
                    best_score = score;
                    best_idx = Some(i);
                    placed = Rect {
                        x: free.x,
                        y: free.y,
                        w,
                        h,
                    };
                }
            }
        }
        best_idx?; // return None if no free rect fits
        let mut splits: Vec<Rect> = Vec::with_capacity(self.free_rects.len() * 2);
        let mut i = 0;
        while i < self.free_rects.len() {
            if Self::overlaps(self.free_rects[i], placed) {
                let free = self.free_rects.swap_remove(i);
                Self::push_splits(free, placed, &mut splits);
                // Do NOT advance i: swap_remove moved the last element to position i.
            } else {
                i += 1;
            }
        }
        self.free_rects.extend(splits);
        // Prune sub-rects first (cheaper), then merge to recombine strips.
        self.prune_free_list();
        self.merge_free_rects();

        Some((placed.x, placed.y))
    }
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
