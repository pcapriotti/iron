pub struct Layout {
    /// Size of unit square, including margin.
    pub unit: u32,
    /// Unit margin.
    pub gap: u32,
    /// Bottom-left corner of the bottom-left tile.
    pub origin: (u32, u32),
    /// Number of tiles on each row.
    pub width: usize,
    /// Number of tiles on each column.
    pub height: usize,
}

impl Layout {
    pub fn compute(
        pixel_width: u32,
        pixel_height: u32,
        width: usize,
        height: usize,
    ) -> Self {
        let unit = std::cmp::min(
            pixel_width / width as u32,
            pixel_height / height as u32,
        );
        let gap = (unit as f32 * 0.07) as u32;
        let display_width = width as u32 * unit;
        let display_height = height as u32 * unit;

        let x0 = (pixel_width - display_width) / 2;
        let y0 = (pixel_height - display_height) / 2;

        Self {
            unit,
            gap,
            origin: (x0, y0),
            width,
            height,
        }
    }
}
