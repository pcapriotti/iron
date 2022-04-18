use crate::animation;
use crate::tiles;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "animation::default_duration_ms")]
    pub animation_duration_ms: u64,

    #[serde(default = "tiles::default_tile_radius")]
    pub tile_radius: f32,

    #[serde(default = "default_size")]
    pub width: usize,

    #[serde(default = "default_size")]
    pub height: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            animation_duration_ms: animation::default_duration_ms(),
            tile_radius: tiles::default_tile_radius(),
            width: default_size(),
            height: default_size(),
        }
    }
}

fn default_size() -> usize {
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let s = r#"
            animation_duration_ms = 1500
        "#;
        let config: Config = toml::from_str(s).unwrap();
        assert_eq!(1500, config.animation_duration_ms);
    }
}
