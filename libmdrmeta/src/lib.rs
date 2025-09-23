use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use toml::value::Value as TomlValue;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Datelike {
    Stringy(String),
    TomlDate(toml::value::Datetime),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Numlike {
    Stringy(String),
    TomlVal(TomlValue),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdrepo_id: Option<String>,

    pub initial: Initial,

    pub software: Software,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_files: Option<RequiredFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_files: Option<Vec<AdditionalFile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proteins: Option<Vec<Protein>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicates: Option<Replicates>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub water: Option<Water>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ligands: Option<Vec<Ligand>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvents: Option<Vec<Solvent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forcefield: Option<Forcefield>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protonation_method: Option<Protonation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestep_information: Option<Timestep>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub papers: Option<Vec<Paper>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Contributor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_permissions: Option<Vec<Permission>>,
}

// --------------------------------------------------
impl Meta {
    //[pyfunction]
    pub fn from_toml(toml: &str) -> Result<Self> {
        let mut meta: Meta = toml::from_str(toml)?;
        meta.fix();
        Ok(meta)
    }

    //[pyfunction]
    pub fn from_json(json: &str) -> Result<Self> {
        let mut meta: Meta = serde_json::from_str(&json)?;
        meta.fix();
        Ok(meta)
    }

    //[pyfunction]
    pub fn from_str(contents: &str) -> Result<Self> {
        let meta = if contents.starts_with("{") {
            Self::from_json(&contents)?
        } else {
            Self::from_toml(&contents)?
        };
        Ok(meta)
    }

    //[pyfunction]
    pub fn from_file(filename: &str) -> Result<Self> {
        match Path::new(filename).extension() {
            Some(ext) => {
                let contents = fs::read_to_string(filename)?;
                if contents.is_empty() {
                    bail!("File is empty")
                }
                let meta = match ext.to_str() {
                    Some("json") => Self::from_json(&contents)?,
                    Some("toml") => Self::from_toml(&contents)?,
                    _ => bail!(r#"Unknown file extension "{}""#, ext.display()),
                };
                Ok(meta)
            }
            _ => bail!("No file extension"),
        }
    }

    //[pyfunction]
    pub fn to_json(self: &Self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(Into::into)
    }

    //[pyfunction]
    pub fn to_toml(self: &Self) -> Result<String> {
        toml::to_string_pretty(&self).map_err(Into::into)
    }

    //[pyfunction]
    pub fn find_errors(&self) -> Vec<(String, String)> {
        let mut errors = vec![];
        if let Some(water) = &self.water {
            if let Some(density) = water.density {
                if density.is_nan() {
                    errors.push((
                        "water.density".to_string(),
                        "cannot be NaN".to_string(),
                    ));
                }
            }
            if !water.is_present {
                if water.model.is_some() {
                    errors.push((
                        "water.model".to_string(),
                        "should not be present if water.is_present is false"
                            .to_string(),
                    ));
                }
                if water.density.is_some() {
                    errors.push((
                        "water.density".to_string(),
                        "should not be present if water.is_present is false"
                            .to_string(),
                    ));
                }
                if water.water_density_units.is_some() {
                    errors.push((
                        "water.water_density_units".to_string(),
                        "should not be present if water.is_present is false"
                            .to_string(),
                    ));
                }
            }
        }
        errors
    }

    fn fix(&mut self) {
        // Some confusion over dates as quoted strings or unquoted TOML values
        // But there's no JSON "date" format
        if let Datelike::TomlDate(dt) = self.initial.date {
            self.initial.date = Datelike::Stringy(dt.to_string())
        }

        if let Some(papers) = &self.papers {
            let new_papers: Vec<_> = papers
                .into_iter()
                .map(|paper| {
                    let volume = if let Numlike::TomlVal(val) = &paper.volume {
                        match val {
                            TomlValue::String(v) => Numlike::Stringy(v.to_string()),
                            TomlValue::Integer(v) => Numlike::Stringy(v.to_string()),
                            TomlValue::Float(v) => Numlike::Stringy(v.to_string()),
                            _ => Numlike::Stringy("".to_string()),
                        }
                    } else {
                        paper.volume.clone()
                    };

                    let number = paper.number.clone().map(|val| {
                        if let Numlike::TomlVal(n) = val {
                            let new_number = match n {
                                TomlValue::String(v) => Numlike::Stringy(v.to_string()),
                                TomlValue::Integer(v) => {
                                    Numlike::Stringy(v.to_string())
                                }
                                TomlValue::Float(v) => Numlike::Stringy(v.to_string()),
                                _ => Numlike::Stringy("".to_string()),
                            };
                            new_number
                        } else {
                            val.clone()
                        }
                    });

                    let mut new_paper = paper.clone();
                    new_paper.volume = volume;
                    new_paper.number = number;
                    new_paper
                })
                .collect();

            self.papers = Some(new_papers);
        }

        // Older versions of the TOML had separate fields for PDB/Uniprot
        if let Some(proteins) = &self.proteins {
            let new_proteins: Vec<_> = proteins
                .into_iter()
                .map(|protein| {
                    if let Some(pdb_id) = &protein.pdb_id {
                        Protein {
                            molecule_id_type: Some("PDB".to_string()),
                            molecule_id: Some(pdb_id.to_string()),
                            pdb_id: None,
                            uniprot_id: None,
                        }
                    } else if let Some(uniprot_id) = &protein.uniprot_id {
                        Protein {
                            molecule_id_type: Some("Uniprot".to_string()),
                            molecule_id: Some(uniprot_id.to_string()),
                            pdb_id: None,
                            uniprot_id: None,
                        }
                    } else {
                        Protein {
                            molecule_id_type: protein.molecule_id_type.clone(),
                            molecule_id: protein.molecule_id.clone(),
                            pdb_id: None,
                            uniprot_id: None,
                        }
                    }
                })
                .collect();

            self.proteins = Some(new_proteins);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Initial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_link: Option<String>,

    pub lead_contributor_orcid: String,

    pub date: Datelike,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_is_restricted: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalFile {
    pub additional_file_type: String,
    pub additional_file_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contributor {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orcid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forcefield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forcefield: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forcefield_comments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Permission {
    pub user_orcid: String,
    pub can_edit: bool,
    pub can_view: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Protonation {
    pub protonation_method: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Timestep {
    pub integration_time_step: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Paper {
    pub title: String,

    pub authors: String,

    pub journal: String,

    pub volume: Numlike,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<Numlike>,

    pub year: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Temperature {
    pub temperature: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Ligand {
    pub name: String,
    pub smiles: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequiredFile {
    pub trajectory_file_name: String,
    pub structure_file_name: String,
    pub topology_file_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Software {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Replicates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_replicates: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicate: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Protein {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecule_id_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecule_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdb_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniprot_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Solvent {
    pub name: String,
    pub ion_concentration: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Water {
    pub is_present: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_density_units: Option<String>,
}

//#[derive(Debug, Serialize)]
//pub struct Error {
//    field: String,
//    error: String,
//}
