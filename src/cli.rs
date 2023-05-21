use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
pub struct UtilCli {
    #[arg(short, long)]
    rom: Option<String>,
    yaml: Option<String>,
    out: Option<String>,
}

impl UtilCli {
    pub fn into_config(self) -> UtilConfig {
        let maybe_rom: String = self.rom.unwrap_or("dash_working.sfc".to_string());
        let rom = PathBuf::from(maybe_rom);

        let maybe_yaml: String = self.yaml.unwrap_or("colors.yaml".to_string());
        let yaml = PathBuf::from(maybe_yaml);

        let maybe_out: String = self.out.unwrap_or("dash_dphu.sfc".to_string());
        let out = PathBuf::from(maybe_out);

        UtilConfig { rom, yaml, out }
    }
}

pub struct UtilConfig {
    pub rom: PathBuf,
    pub yaml: PathBuf,
    pub out: PathBuf,
}
