#[derive(Debug)]
pub struct AppConfig {
    pub server_port: u16,
    pub max_connections: usize,
}

// 提供默认配置
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            server_port: 8080,
            max_connections: 100,
        }
    }
}
