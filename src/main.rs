mod config;

use config::config::Config;
use config::consts::{SERVER_CONFIG_DIR, SERVER_CONFIG_FILE};

fn get_configuration() -> anyhow::Result<Config> {
    let config_file_path: std::path::PathBuf = match dirs_next::config_dir() {
        Some(config_dir) => config_dir.join(SERVER_CONFIG_DIR).join(SERVER_CONFIG_FILE),
        None => {
            return Err(anyhow::anyhow!("Failed to get config directory"));
        }
    };

    if !config_file_path.exists() {
        config::config::create_config_file(&config_file_path)?;
        return Ok(Config::default());
    }
    Config::from_file(&config_file_path)
}

fn main() {
    let config: Config = get_configuration().unwrap();
}
