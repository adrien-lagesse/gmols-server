mod compound;
use anyhow;
use anyhow::Context;
pub use compound::Compound;
use reqwest;
use serde_json::Value as JSON;

const PUBCHEM_PROLOG: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/";

pub struct Client {
    reqwest_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        let client = reqwest::Client::builder().build().unwrap();
        Client { reqwest_client: client }
    }

    pub async fn get_compound_by_cid(&self, cid: u64) -> anyhow::Result<compound::Compound> {
        let response = self
            .reqwest_client
            .get(
                format!("{PUBCHEM_PROLOG}/compound/cid/{cid}/property/MolecularFormula,MolecularWeight,CanonicalSMILES,IUPACName,Title/JSON")
            )
            .send()
            .await?;

        let result: JSON = response.json().await?;

        let name = result
            .pointer("/PropertyTable/Properties/0/Title")
            .and_then(|s| s.as_str())
            .context("Can't find title")?;
        let iupac_name = result
            .pointer("/PropertyTable/Properties/0/IUPACName")
            .and_then(|s| s.as_str())
            .context("Can't find IUPACName")?;
        let molecular_formula = result
            .pointer("/PropertyTable/Properties/0/MolecularFormula")
            .and_then(|s| s.as_str())
            .context("Can't find MolecularFormula")?;
        let canonical_smiles = result
            .pointer("/PropertyTable/Properties/0/CanonicalSMILES")
            .and_then(|s| s.as_str())
            .context("Can't find CanonicalSMILES")?;
        let molecular_weight: f32 = result
            .pointer("/PropertyTable/Properties/0/MolecularWeight")
            .and_then(|s| s.as_str())
            .and_then(|s| s.parse().ok())
            .context("Can't find MolecularWeight")?;

        let response = self
            .reqwest_client
            .get(format!("{PUBCHEM_PROLOG}/compound/cid/{cid}/description/JSON"))
            .send()
            .await?;

        let result: JSON = response.json().await?;

        let description = result
            .pointer("/InformationList/Information/1/Description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Compound::new(
            cid,
            name.to_string(),
            iupac_name.to_string(),
            description,
            molecular_formula.to_string(),
            canonical_smiles.to_string(),
            molecular_weight,
        ))
    }
}
