use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
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

impl Meta {
    pub fn from_file(filename: &str) -> Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let mut toml: Meta = toml::from_str(&contents)?;
        toml.fix();
        Ok(toml)
    }

    pub fn fix(&mut self) {
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

    pub fn find_errors(&self) -> Vec<String> {
        let mut errors = vec![];
        if let Some(water) = &self.water {
            if let Some(density) = water.density {
                if density.is_nan() {
                    errors.push("water.density cannot be NaN".to_string());
                }
            }
            if !water.is_present {
                if water.model.is_some() {
                    errors.push(
                        "water.model should not be present if water.is_present is false"
                            .to_string(),
                    );
                }
                if water.density.is_some() {
                    errors.push(
                        "water.density should not be present if water.is_present is false"
                            .to_string(),
                    );
                }
                if water.water_density_units.is_some() {
                    errors.push(
                        "water.water_density_units should not be present\
                        if water.is_present is false"
                            .to_string(),
                    );
                }
            }
        }
        errors
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

#[cfg(test)]
mod tests {
    const INPUT1: &str = "tests/inputs/MDR_00000002.toml";
    const INPUT2: &str = "tests/inputs/MDR_00004423.toml";
    use super::{Datelike, Ligand, Meta, Protein};
    use anyhow::Result;
    use std::fs;
    use toml;

    #[test]
    fn t1() -> Result<()> {
        let toml = fs::read_to_string(INPUT1)?;
        let mut meta: Meta = toml::from_str(&toml)?;
        meta.fix();

        assert_eq!(
            meta.initial.date,
            Datelike::Stringy("2020-07-13".to_string())
        );

        assert!(meta.proteins.is_some());
        let proteins = meta.proteins.unwrap();
        assert_eq!(proteins.len(), 1);
        assert_eq!(
            proteins[0],
            Protein {
                molecule_id_type: Some("PDB".to_string()),
                molecule_id: Some("1U19.A".to_string()),
                pdb_id: None,
                uniprot_id: None,
            }
        );

        assert!(meta.solvents.is_some());
        let solvents = meta.solvents.unwrap();
        assert_eq!(solvents.len(), 2);

        assert!(meta.papers.is_some());
        let papers = meta.papers.unwrap();
        assert_eq!(papers.len(), 2);

        assert!(meta.contributors.is_some());
        let contributors = meta.contributors.unwrap();
        assert_eq!(contributors.len(), 1);

        assert!(meta.replicates.is_some());
        let replicates = meta.replicates.unwrap();
        assert_eq!(replicates.replicate, Some(2));
        assert_eq!(replicates.total_replicates, Some(3));

        assert_eq!(meta.software.name, "ACEMD".to_string());
        assert_eq!(meta.software.version, Some("GPUGRID".to_string()));

        assert!(meta.water.is_some());
        let water = meta.water.unwrap();
        assert_eq!(water.is_present, true);

        Ok(())
    }

    #[test]
    fn t2() -> Result<()> {
        let toml = fs::read_to_string(INPUT2)?;
        let mut meta: Meta = toml::from_str(&toml)?;
        meta.fix();

        assert_eq!(
            meta.initial.date,
            Datelike::Stringy("2024-09-20".to_string())
        );

        assert!(meta.initial.commands.is_some());
        let commands = meta.initial.commands.unwrap();
        assert!(commands.starts_with("gmx_mpi"));
        assert!(commands.ends_with("gpu"));

        assert!(meta.replicates.is_some());
        let replicates = meta.replicates.unwrap();
        assert_eq!(replicates.replicate, Some(1));
        assert_eq!(replicates.total_replicates, Some(4));

        assert!(meta.proteins.is_some());
        let proteins = meta.proteins.unwrap();
        assert_eq!(proteins.len(), 1);
        assert_eq!(
            proteins[0],
            Protein {
                molecule_id_type: Some("PDB".to_string()),
                molecule_id: Some("5UPE".to_string()),
                pdb_id: None,
                uniprot_id: None,
            }
        );

        assert!(meta.ligands.is_some());
        let ligands = meta.ligands.unwrap();
        assert_eq!(ligands.len(), 1);
        assert_eq!(
            ligands[0],
            Ligand {
                name:
                    "N-{4-[(3-phenylpropyl)carbamoyl]phenyl}-2H-isoindole-2-carboxamide"
                        .to_string(),
                smiles: "c1ccc(cc1)CCCNC(=O)c2ccc(cc2)NC(=O)n3cc4ccccc4c3".to_string()
            }
        );

        assert!(meta.solvents.is_some());
        let solvents = meta.solvents.unwrap();
        assert_eq!(solvents.len(), 1);

        assert!(meta.papers.is_none());

        assert!(meta.contributors.is_some());
        let contributors = meta.contributors.unwrap();
        assert_eq!(contributors.len(), 1);

        assert!(meta.forcefield.is_some());
        let forcefield = meta.forcefield.unwrap();
        assert_eq!(forcefield.forcefield, Some("charmm36".to_string()));
        assert_eq!(
            forcefield.forcefield_comments,
            Some("ligand parameters from swissparam".to_string())
        );

        assert!(meta.temperature.is_some());
        let temperature = meta.temperature.unwrap();
        assert_eq!(temperature.temperature, Some(300));

        assert_eq!(meta.software.name, "GROMACS".to_string());
        assert_eq!(meta.software.version, Some("2024".to_string()));

        assert!(meta.water.is_some());
        let water = meta.water.unwrap();
        assert_eq!(water.is_present, true);
        assert_eq!(water.model, Some("TIP3P".to_string()));
        assert_eq!(water.density, Some(1000.));
        assert_eq!(water.water_density_units, Some("g/m^3".to_string()));

        assert!(meta.required_files.is_some());
        let required_files = meta.required_files.unwrap();
        assert_eq!(required_files.trajectory_file_name, "prodw.xtc".to_string());
        assert_eq!(
            required_files.structure_file_name,
            "prod.part0135.pdb".to_string()
        );
        assert_eq!(required_files.topology_file_name, "prod.tpr".to_string());

        assert!(meta.additional_files.is_some());
        let additional_files = meta.additional_files.unwrap();
        assert_eq!(additional_files.len(), 9);

        Ok(())
    }
}
