mod metadata;

use crate::metadata::Meta;
use anyhow::Result;

// --------------------------------------------------
pub fn validate(filename: &str) -> Result<()> {
    let meta = Meta::from_file(filename)?;
    dbg!(&meta);
    let errors = &meta.find_errors();
    if !errors.is_empty() {
        eprintln!("{}", errors.join("\n"));
    }

    Ok(())
}

// --------------------------------------------------
pub fn to_json(filename: &str) -> Result<String> {
    let meta = Meta::from_file(filename)?;
    let json = serde_json::to_string_pretty(&meta)?;
    Ok(json)
}

// --------------------------------------------------
pub fn to_toml(filename: &str) -> Result<String> {
    let mut meta = Meta::from_file(filename)?;
    meta.fix();
    let toml = toml::to_string_pretty(&meta)?;
    Ok(toml)
}
