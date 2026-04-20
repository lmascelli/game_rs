use configini::Configini;

#[derive(Debug, Clone, Configini)]
pub struct AppConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub default_tile_width: u32,
    pub default_tile_height: u32,
    pub with_dgpu: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            default_tile_width: 32,
            default_tile_height: 32,
            with_dgpu: false,
        }
    }
}
