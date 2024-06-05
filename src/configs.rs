use dirs::config_dir;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub cyan: CyanConfigs,
}

#[derive(Debug, Deserialize)]
pub struct CyanConfigs {
    pub delete_after_sync: bool,
}

impl Configs {
    pub fn get() -> Self {
        Figment::new()
            .merge(Env::prefixed("CYAN"))
            .merge(Toml::file(format!(
                "{}configs.toml",
                get_configs_dir().unwrap()
            )))
            .extract()
            .expect("Failed to load configs")
    }
}

pub fn get_configs_dir() -> anyhow::Result<String> {
    let dir = if cfg!(debug_assertions) {
        "".to_owned()
    } else {
        format!(
            "{}/cyan/",
            config_dir().unwrap().to_str().unwrap().to_owned()
        )
    };

    std::fs::create_dir_all(&dir)?;

    Ok(dir)
}
