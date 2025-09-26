use anyhow::Result;
use libmdrmeta::metav1::{Datelike, Ligand, MetaV1, Protein};
use pretty_assertions::assert_eq;
use std::fs;

const BAD_JSON: &str = "../tests/inputs/bad.json";
const BAD_TOML: &str = "../tests/inputs/bad.toml";
const EMPTY: &str = "../tests/inputs/empty";
const EMPTY_JSON: &str = "../tests/inputs/empty.json";
const EMPTY_TOML: &str = "../tests/inputs/empty.toml";
const FULL_EXAMPLE: &str = "../tests/inputs/example.toml";
const MDR0002_JSON: &str = "../tests/inputs/MDR_00000002.json";
const MDR0002_TOML: &str = "../tests/inputs/MDR_00000002.toml";
const MDR4423_TOML: &str = "../tests/inputs/MDR_00004423.toml";
const OUTPUT_MDR0002_JSON: &str = "../tests/outputs/MDR_00000002.json";
const OUTPUT_MDR0002_TOML: &str = "../tests/outputs/MDR_00000002.toml";

// --------------------------------------------------
#[test]
fn dies_from_file_no_ext() -> Result<()> {
    let res = MetaV1::from_file(EMPTY);
    assert!(!res.is_ok());

    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "No file extension");

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_from_file_bad() -> Result<()> {
    let res = MetaV1::from_file(EMPTY_TOML);
    assert!(!res.is_ok());

    let res = MetaV1::from_file(EMPTY_JSON);
    assert!(!res.is_ok());

    let res = MetaV1::from_file(BAD_TOML);
    assert!(!res.is_ok());

    let res = MetaV1::from_file(BAD_JSON);
    assert!(!res.is_ok());

    Ok(())
}

// --------------------------------------------------
#[test]
fn from_file_toml() -> Result<()> {
    let res = MetaV1::from_file(MDR0002_TOML);
    assert!(res.is_ok());

    let meta = res.unwrap();
    let desc = meta.initial.description.expect("description");
    assert!(desc.starts_with("Rhodopsin"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn from_file_json() -> Result<()> {
    let res = MetaV1::from_file(MDR0002_JSON);
    assert!(res.is_ok());

    let meta = res.unwrap();
    let desc = meta.initial.description.expect("description");
    assert!(desc.starts_with("Rhodopsin"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn from_json() -> Result<()> {
    let contents = fs::read_to_string(MDR0002_JSON)?;
    let res = MetaV1::from_json(&contents);
    assert!(res.is_ok());

    let meta = res.unwrap();
    let desc = meta.initial.description.expect("description");
    assert!(desc.starts_with("Rhodopsin"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn from_toml() -> Result<()> {
    let contents = fs::read_to_string(MDR0002_TOML)?;
    let res = MetaV1::from_toml(&contents);
    assert!(res.is_ok());

    let meta = res.unwrap();
    let desc = meta.initial.description.expect("description");
    assert!(desc.starts_with("Rhodopsin"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn from_str_toml() -> Result<()> {
    let contents = fs::read_to_string(MDR0002_TOML)?;
    let res = MetaV1::from_string(&contents);
    assert!(res.is_ok());

    let meta = res.unwrap();
    let desc = meta.initial.description.expect("description");
    assert!(desc.starts_with("Rhodopsin"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn from_str_json() -> Result<()> {
    let contents = fs::read_to_string(MDR0002_JSON)?;
    let res = MetaV1::from_string(&contents);
    assert!(res.is_ok());

    let meta = res.unwrap();
    let desc = meta.initial.description.expect("description");
    assert!(desc.starts_with("Rhodopsin"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn toml_to_toml() -> Result<()> {
    let meta = MetaV1::from_file(MDR0002_TOML)?;
    let expected = fs::read_to_string(OUTPUT_MDR0002_TOML)?;
    let actual = meta.to_toml()?;
    assert_eq!(actual, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn toml_to_json() -> Result<()> {
    let meta = MetaV1::from_file(MDR0002_TOML)?;
    let expected = fs::read_to_string(OUTPUT_MDR0002_JSON)?;
    let actual = meta.to_json()?;
    assert_eq!(actual, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn json_to_toml() -> Result<()> {
    let meta = MetaV1::from_file(MDR0002_JSON)?;
    let expected = fs::read_to_string(OUTPUT_MDR0002_TOML)?;
    let actual = meta.to_toml()?;
    assert_eq!(actual, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn parses_0002() -> Result<()> {
    let meta = MetaV1::from_file(MDR0002_TOML)?;

    assert_eq!(
        meta.initial.date,
        Datelike::Stringy("2020-07-13".to_string())
    );

    assert!(meta.proteins.is_some());
    let proteins = meta.proteins.unwrap();
    assert_eq!(proteins.len(), 1);
    assert_eq!(
        proteins[0],
        Protein::ProteinNew {
            primary: None,
            molecule_id_type: "PDB".to_string(),
            molecule_id: "1U19.A".to_string(),
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

// --------------------------------------------------
#[test]
fn parses_4423() -> Result<()> {
    let toml = fs::read_to_string(MDR4423_TOML)?;
    let meta: MetaV1 = toml::from_str(&toml)?;

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
        Protein::ProteinNew {
            primary: None,
            molecule_id_type: "PDB".to_string(),
            molecule_id: "5UPE".to_string(),
        }
    );

    assert!(meta.ligands.is_some());
    let ligands = meta.ligands.unwrap();
    assert_eq!(ligands.len(), 1);
    assert_eq!(
        ligands[0],
        Ligand {
            primary: None,
            name: "N-{4-[(3-phenylpropyl)carbamoyl]phenyl}-2H-isoindole-2-carboxamide"
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

// --------------------------------------------------
#[test]
fn parses_full_example() -> Result<()> {
    let meta = MetaV1::from_file(FULL_EXAMPLE)?;

    assert_eq!(meta.initial.date.to_string(), "2000-02-05");

    if let Some(papers) = &meta.papers {
        assert_eq!(papers.len(), 2);
    }

    if let Some(contributors) = &meta.contributors {
        assert_eq!(contributors.len(), 2);
    }

    if let Some(files) = &meta.additional_files {
        assert_eq!(files.len(), 2);
    }

    if let Some(ligands) = &meta.ligands {
        assert_eq!(ligands.len(), 2);
    }

    if let Some(solvents) = &meta.solvents {
        assert_eq!(solvents.len(), 2);
    }

    if let Some(proteins) = &meta.proteins {
        assert_eq!(proteins.len(), 2);
    }

    if let Some(perms) = &meta.simulation_permissions {
        assert_eq!(perms.len(), 2);
    }

    Ok(())
}
