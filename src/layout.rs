use crate::game::Move;

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

    pub fn animate(
        &self,
        moves: &[Move],
        x: usize,
        y: usize,
        time: f32,
    ) -> (i32, i32) {
        if let Some(mv_index) =
            moves.iter().position(|m| m.dst == x + y * self.width)
        {
            let mv = &moves[mv_index];

            let src_point = (mv.src % self.width, mv.src / self.width);
            let dst_point = (x, y);
            let delta_x = ((dst_point.0 as f32 - src_point.0 as f32)
                * self.unit as f32
                * (1.0 - time)) as i32;
            let delta_y = ((dst_point.1 as f32 - src_point.1 as f32)
                * self.unit as f32
                * (1.0 - time)) as i32;
            (delta_x, delta_y)
        } else {
            (0, 0)
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
