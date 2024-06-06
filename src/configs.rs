use dirs::config_dir;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub cyan: CyanConfigs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CyanConfigs {
    pub delete_after_sync: bool,
    pub hash_seed: i64,
}

impl Configs {
    pub fn get() -> Self {
        Figment::from(Serialized::defaults(Configs::default()))
            .merge(Env::prefixed("CYAN"))
            .merge(Toml::file(format!(
                "{}configs.toml",
                get_configs_dir().unwrap()
            )))
            .extract()
            .expect("Failed to load configs")
    }
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            cyan: CyanConfigs {
                delete_after_sync: false,
                //01158544
                hash_seed: 1158544,
            },
        }
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
