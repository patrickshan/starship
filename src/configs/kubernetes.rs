use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;
use toml::Value;

#[derive(Clone, ModuleConfig)]
pub struct KubernetesConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub environments: Vec<KubernetesEnvironmentConfig<'a>>,
    pub context_aliases: HashMap<String, &'a str>,
}

impl<'a> RootModuleConfig<'a> for KubernetesConfig<'a> {
    fn new() -> Self {
        KubernetesConfig {
            symbol: "☸ ",
            format: "on [$symbol$context( \\($namespace\\))]($style) ",
            style: "cyan bold",
            disabled: true,
            environments: Vec::new(),
            context_aliases: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct KubernetesEnvironmentConfig<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
}

impl<'a> ModuleConfig<'a> for KubernetesEnvironmentConfig<'a> {
    fn from_config(config: &'a Value) -> Option<Self> {
        let table = config.as_table()?;
        let env_symbol = match table.get("symbol") {
            Some(s) => <&str>::from_config(s)?,
            None => "",
        };
        let env_style = match table.get("style") {
            Some(s) => <&str>::from_config(s)?,
            None => "",
        };

        Some(KubernetesEnvironmentConfig {
            name: table.get("name").and_then(|v| <&str>::from_config(v))?,
            symbol: env_symbol,
            style: env_style,
        })
    }
}
