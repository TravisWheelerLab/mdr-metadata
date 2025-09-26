use crate::{
    common::{Datelike, Numlike, RequiredFile, Software, MAX_TEMP_K, MIN_TEMP_K},
    metav2::MetaV2,
};
use anyhow::{anyhow, bail, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use toml::value::Value as TomlValue;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetaV1 {
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scientific_goal: Option<String>,

    // TODO: Remove?
    // These are only here because people put them in the wrong place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ligands: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvents: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdditionalFile {
    pub additional_file_type: String,

    pub additional_file_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_file_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct Forcefield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forcefield: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forcefield_comments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Permission {
    pub user_orcid: String,

    pub can_edit: bool,

    pub can_view: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Protonation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protonation_method: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Timestep {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_time_step: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Paper {
    #[serde(skip_serializing_if = "Option::is_none", alias = "primary")]
    pub primary: Option<bool>,

    pub title: String,

    pub authors: String,

    pub journal: String,

    pub volume: Numlike,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<Numlike>,

    pub year: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Temperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Ligand {
    #[serde(skip_serializing_if = "Option::is_none", alias = "primary")]
    pub primary: Option<bool>,

    pub name: String,

    pub smiles: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Replicates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_replicates: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicate: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Protein {
    ProteinOldPDB {
        #[serde(skip_serializing_if = "Option::is_none")]
        primary: Option<bool>,

        pdb_id: String,
    },
    ProteinOldUniprot {
        #[serde(skip_serializing_if = "Option::is_none")]
        primary: Option<bool>,

        uniprot_id: String,
    },
    ProteinNew {
        #[serde(skip_serializing_if = "Option::is_none")]
        primary: Option<bool>,

        molecule_id_type: String,

        molecule_id: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Solvent {
    pub name: String,

    pub ion_concentration: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvent_concentration_units: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Water {
    pub is_present: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_density_units: Option<String>,
}

impl MetaV1 {
    //[pyfunction]
    pub fn from_toml(toml: &str) -> Result<Self> {
        let mut meta: Self = toml::from_str(toml)?;
        meta.to_canon()?;
        Ok(meta)
    }

    //[pyfunction]
    pub fn from_json(json: &str) -> Result<Self> {
        let mut meta: Self = serde_json::from_str(json)?;
        meta.to_canon()?;
        Ok(meta)
    }

    //[pyfunction]
    pub fn from_string(contents: &str) -> Result<Self> {
        let meta = if contents.starts_with("{") {
            Self::from_json(contents)?
        } else {
            Self::from_toml(contents)?
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
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(Into::into)
    }

    //[pyfunction]
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(&self).map_err(Into::into)
    }

    pub fn to_v2(&self) -> Result<MetaV2> {
        let meta_v2 = MetaV2 {
            mdrepo_id: self.mdrepo_id.clone(),
            short_description: self.initial.short_description.clone(),
            description: self.initial.description.clone(),
            external_link: self.initial.external_link.clone(),
            lead_contributor_orcid: self.initial.lead_contributor_orcid.clone(),
            date: self.initial.date.clone(),
            run_commands: self.initial.commands.clone(),
            software: self.software.clone(),
            replicate_id: self.replicates.map_or(None, |rep| rep.replicate.clone()),
            total_replicates: self
                .replicates
                .map_or(None, |rep| rep.total_replicates.clone()),
            water_is_present: self.water.map_or(None, |water| Some(water.is_present)),
            water_model: self.water.map_or(None, |water| water.model.clone()),
            water_density_kg_m3: self.water.map_or(None, |water| water.density.clone()),
            forcefield: self.forcefield.map_or(None, |f| f.forcefield.clone()),
            forcefield_comments: self
                .forcefield
                .map_or(None, |f| f.forcefield_comments.clone()),
            temperature_kelvin: self.temperature.map_or(None, |t| t.temperature),
            protonation_method: self
                .protonation_method
                .map_or(None, |p| p.protonation_method),
            timestep_ns: self
                .timestep_information
                .map_or(None, |ts| ts.integration_time_step),
            required_file: self.required_files.unwrap().clone(),
            // TODO: pick up here!
        };

        Ok(meta_v2)
    }

    //[pyfunction]
    pub fn find_errors(&self) -> Vec<(String, String)> {
        let mut errors = vec![];
        //if let Some(replicates) = &self.replicates {
        //    if replicates.replicate.unwra
        //}

        if let Some(temp) = &self.temperature.clone().and_then(|t| t.temperature) {
            if !(MIN_TEMP_K..=MAX_TEMP_K).contains(temp) {
                errors.push((
                    "temperature.temperature".to_string(),
                    format!(
                        r#""{temp}" must be in the range {MIN_TEMP_K}-{MAX_TEMP_K}"#
                    ),
                ))
            }
        }

        let valid_date = Regex::new(r"\d{4}\-\d{2}\-\d{2}").unwrap();
        match &self.initial.date {
            Datelike::Stringy(dt) => {
                if !valid_date.is_match(dt) {
                    errors.push((
                        "initial.date".to_string(),
                        format!(r#"invalid date "{}""#, dt),
                    ));
                }
            }
            _ => {
                errors.push(("initial.date".to_string(), "invalid date".to_string()));
            }
        }

        fn is_valid_orcid(orcid: &str) -> bool {
            let re = Regex::new(r"\d{4}\-\d{4}\-\d{4}\-\d{3}[A-Z]").unwrap();
            re.is_match(orcid)
        }

        if !is_valid_orcid(&self.initial.lead_contributor_orcid) {
            errors.push((
                "initial.lead_contributor_orcid".to_string(),
                format!(r#"invalid ORCID "{}""#, self.initial.lead_contributor_orcid),
            ));
        }

        if let Some(contributors) = &self.contributors {
            for contributor in contributors {
                if let Some(orcid) = &contributor.orcid {
                    if !is_valid_orcid(orcid) {
                        errors.push((
                            "contributor.orcid".to_string(),
                            format!(r#"invalid ORCID "{}""#, orcid),
                        ));
                    }
                }
            }
        }

        if let Some(perms) = &self.simulation_permissions {
            for perm in perms {
                if !is_valid_orcid(&perm.user_orcid) {
                    errors.push((
                        "simulation_permissions.user_orcid".to_string(),
                        format!(r#"invalid ORCID "{}""#, perm.user_orcid),
                    ));
                }
            }
        }

        if let Some(water) = &self.water {
            if let Some(density) = water.density {
                if !density.is_finite() {
                    errors.push((
                        "water.density".to_string(),
                        format!("{density} is not a finite value"),
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

        if let Some(solvents) = &self.solvents {
            for solvent in solvents {
                if !solvent.ion_concentration.is_finite() {
                    errors.push((
                        "solvent.ion_concentration".to_string(),
                        format!(
                            "{:?} is not a finite value",
                            solvent.ion_concentration
                        ),
                    ));
                }
            }
        }

        if let Some(timestep) = &self.timestep_information {
            if timestep
                .integration_time_step
                .map_or(false, |val| !val.is_finite())
            {
                errors.push((
                    "timestep.integration_time_step".to_string(),
                    format!(
                        "{:?} is not a finite value",
                        timestep.integration_time_step.unwrap()
                    ),
                ));
            }
        }

        errors
    }

    fn to_canon(&mut self) -> Result<()> {
        // Some confusion over dates as quoted strings or unquoted TOML values
        // But there's no JSON "date" format
        let date = self.initial.date.to_string();
        let dt = dateparser::parse_with_timezone(&date, &chrono::offset::Utc)
            .map_err(|e| anyhow!(r#"initial.date {e}"#))?;
        self.initial.date = Datelike::Stringy(format!("{}", dt.format("%F")));

        // TODO: This is silly, but I'll have to do the same for the "solvents"?
        if let Some(initial_ligands) = &self.initial.ligands {
            if let Some(ligands) = &mut self.ligands {
                for ligand_name in initial_ligands {
                    ligands.push(Ligand {
                        primary: None,
                        name: ligand_name.clone(),
                        smiles: "".to_string(),
                    });
                }
            } else {
                self.ligands = Some(
                    initial_ligands
                        .iter()
                        .map(|ligand_name| Ligand {
                            primary: None,
                            name: ligand_name.clone(),
                            smiles: "".to_string(),
                        })
                        .collect(),
                );
            }
        }
        self.initial.ligands = None;

        if let Some(papers) = &self.papers {
            let new_papers: Vec<_> = papers
                .iter()
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
                            match n {
                                TomlValue::String(v) => Numlike::Stringy(v.to_string()),
                                TomlValue::Integer(v) => {
                                    Numlike::Stringy(v.to_string())
                                }
                                TomlValue::Float(v) => Numlike::Stringy(v.to_string()),
                                _ => Numlike::Stringy("".to_string()),
                            }
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
                .iter()
                .map(|protein| match protein {
                    Protein::ProteinOldPDB { primary, pdb_id } => Protein::ProteinNew {
                        primary: primary.clone(),
                        molecule_id_type: "PDB".to_string(),
                        molecule_id: pdb_id.clone(),
                    },
                    Protein::ProteinOldUniprot {
                        primary,
                        uniprot_id,
                    } => Protein::ProteinNew {
                        primary: primary.clone(),
                        molecule_id_type: "Uniprot".to_string(),
                        molecule_id: uniprot_id.clone(),
                    },
                    _ => protein.clone(),
                })
                .collect();

            self.proteins = Some(new_proteins);
        }
        Ok(())
    }

    // Create an example with every field with valid values
    pub fn example() -> Self {
        Self {
            initial: Initial {
                short_description: Some(
                    "Adaptive sampling of AncFT luciferase".to_string(),
                ),
                description: Some(
                    "Adaptive sampling of AncFT luciferase performed in \
                    HTMD, using a C-alpha RMSD metric. 5 microseconds in total. 10 \
                    epochs of 10 parallel simulations each."
                        .to_string(),
                ),
                external_link: Some("http://external.link".to_string()),
                lead_contributor_orcid: "0000-0000-0000-000X".to_string(),
                date: Datelike::Stringy("2000-01-01".to_string()),
                commands: Some(
                    "gmx_mpi mdrun -s fname.tpr -deffnm fname -v -c fname.pdb \
                    -cpi fname.cpt -maxh clock_time -noappend -update gpu -bonded gpu \
                    -pme gpu -pmefft gpu -nb gpu"
                        .to_string(),
                ),
                simulation_is_restricted: Some(false),
                scientific_goal: None,
                ligands: None,
                solvents: None,
            },
            required_files: Some(RequiredFile {
                trajectory_file_name: "trajectory.xtc".to_string(),
                structure_file_name: "structure.pdb".to_string(),
                topology_file_name: "topology.psf".to_string(),
            }),
            additional_files: Some(vec![
                AdditionalFile {
                    additional_file_type: "Checkpoint".to_string(),
                    additional_file_name: "abc.cpt".to_string(),
                    additional_file_description: Some(
                        "Last GROMACS checkpoint of the \
                        simulation"
                            .to_string(),
                    ),
                },
                AdditionalFile {
                    additional_file_type: "Miscellaneous".to_string(),
                    additional_file_name: "xyz.tpr".to_string(),
                    additional_file_description: None,
                },
            ]),
            contributors: Some(vec![
                Contributor {
                    name: "Contributor1".to_string(),
                    orcid: Some("0000-0000-0000-000X".to_string()),
                    email: Some("email@place.edu".to_string()),
                    institution: Some("Institution".to_string()),
                },
                Contributor {
                    name: "Contributor2".to_string(),
                    orcid: Some("0000-0000-0000-000X".to_string()),
                    email: Some("email@anotherplace.edu".to_string()),
                    institution: Some("Some Other Institution".to_string()),
                },
            ]),
            forcefield: Some(Forcefield {
                forcefield: Some("Amber99SB-ILDN".to_string()),
                forcefield_comments: Some("ligand params: GAFF".to_string()),
            }),
            ligands: Some(vec![
                Ligand {
                    primary: None,
                    name: "Foropafant".to_string(),
                    smiles: "CC(C)C1=CC(=C(C(=C1)C(C)C)C2=CSC(=N2)N(CCN(C)C)\
                        CC3=CN=CC=C3)C(C)C"
                        .to_string(),
                },
                Ligand {
                    primary: None,
                    name: "Vipadenant".to_string(),
                    smiles: "CC1=C(C=CC(=C1)CN2C3=NC(=NC(=C3N=N2)C4=CC=CO4)N)N"
                        .to_string(),
                },
            ]),
            mdrepo_id: None,
            papers: Some(vec![
                Paper {
                    primary: Some(true),
                    title: "GPCRmd uncovers the dynamics of the 3D-GPCRome".to_string(),
                    authors: "Rodríguez, I., Fontanals, M., Tielmann, J.S. et al."
                        .to_string(),
                    journal: "Nat Methods".to_string(),
                    volume: Numlike::Stringy("17".to_string()),
                    number: Some(Numlike::Stringy("4".to_string())),
                    year: 2000,
                    pages: Some("777–787".to_string()),
                    doi: Some("10.1038/x41594-020-0884-y".to_string()),
                },
                Paper {
                    primary: None,
                    title: "Adrenaline-activated structure of β2-adrenoceptor \
                        stabilized by an engineered nanobody"
                        .to_string(),
                    authors: "Ring, A., Manglik, A., Kruse, A., Enos, M., Weis, \
                        W., Garcia, K., Kobilka, B."
                        .to_string(),
                    journal: "Nature".to_string(),
                    volume: Numlike::Stringy("502".to_string()),
                    number: Some(Numlike::Stringy("7472".to_string())),
                    year: 2013,
                    pages: Some("575-579".to_string()),
                    doi: Some("10.1038/nature12572".to_string()),
                },
            ]),
            proteins: Some(vec![
                Protein::ProteinNew {
                    primary: None,
                    molecule_id_type: "PDB".to_string(),
                    molecule_id: "7QXR".to_string(),
                },
                Protein::ProteinNew {
                    primary: None,
                    molecule_id_type: "Uniprot".to_string(),
                    molecule_id: "A7M120".to_string(),
                },
            ]),
            protonation_method: Some(Protonation {
                protonation_method: Some("PROPKA".to_string()),
            }),
            replicates: Some(Replicates {
                replicate: Some(1),
                total_replicates: Some(10),
            }),
            simulation_permissions: Some(vec![
                Permission {
                    user_orcid: "0000-0000-0000-000X".to_string(),
                    can_edit: true,
                    can_view: false,
                },
                Permission {
                    user_orcid: "0000-0000-0000-001X".to_string(),
                    can_edit: false,
                    can_view: true,
                },
            ]),
            software: Software {
                name: "GROMACS".to_string(),
                version: Some("2016.5".to_string()),
            },
            solvents: Some(vec![
                Solvent {
                    name: "Sodium".to_string(),
                    ion_concentration: 0.157,
                    solvent_concentration_units: Some("mol/L".to_string()),
                },
                Solvent {
                    name: "Chloride".to_string(),
                    ion_concentration: 0.225,
                    solvent_concentration_units: Some("mol/L".to_string()),
                },
            ]),
            temperature: Some(Temperature {
                temperature: Some(273),
            }),
            timestep_information: Some(Timestep {
                integration_time_step: Some(2.),
            }),
            water: Some(Water {
                is_present: true,
                model: Some("TIP3P".to_string()),
                density: Some(0.986),
                water_density_units: Some("g/m^3".to_string()),
            }),
        }
    }
}
