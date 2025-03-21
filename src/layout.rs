pub struct Layout {
    /// Size of unit square, including margin.
    pub unit: u32,
    /// Unit margin.
    pub gap: u32,
    /// Bottom-left corner of the bottom-left tile.
    pub origin: (u32, u32),
    /// Size of the whole viewport, in pixels
    pub size: (u32, u32),
    /// Number of tiles on each row.
    pub width: usize,
}

impl Layout {
    pub fn compute(pixel_width: u32, pixel_height: u32, width: usize, height: usize) -> Self {
        let unit = std::cmp::min(pixel_width / width as u32, pixel_height / height as u32);
        let gap = (unit as f32 * 0.07) as u32;
        let display_width = width as u32 * unit;
        let display_height = height as u32 * unit;

        let x0 = (pixel_width - display_width) / 2;
        let y0 = (pixel_height - display_height) / 2;

        Self {
            unit,
            gap,
            origin: (x0, y0),
            size: (display_width, display_height),
            width,
        }
    }

    pub fn rect(&self, pos: (usize, usize)) -> [u32; 4] {
        [
            self.origin.0 + pos.0 as u32 * self.unit + self.gap,
            self.origin.1 + pos.1 as u32 * self.unit + self.gap,
            self.unit - 2 * self.gap,
            self.unit - 2 * self.gap,
        ]
    }
}
