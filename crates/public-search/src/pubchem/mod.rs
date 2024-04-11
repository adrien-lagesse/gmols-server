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

    pub async fn get_compound_by_cid(&self, cid: u32) -> anyhow::Result<compound::Compound> {
        let response = self
            .reqwest_client
            .get(
                format!("{PUBCHEM_PROLOG}/compound/cid/{cid}/property/MolecularFormula,MolecularWeight,CanonicalSMILES,IUPACName,Title/JSON")
            )
            .send()
            .await?;

        let result: JSON = response.json().await?;
        let properties = result
            .as_object()
            .context("JSON object is not well formatted")?
            .get("PropertyTable")
            .context("No PropertyTable")?
            .as_object()
            .context("JSON object is not well formatted")?
            .get("Properties")
            .context("No Properties")?
            .as_array()
            .context("JSON object is not well formatted")?[0]
            .as_object()
            .context("JSON object is not well formatted")?;

        let name = properties.get("Title").context("No Title")?.as_str().context("Title no str")?;
        let iupac_name = properties
            .get("IUPACName")
            .context("No IUPACName")?
            .as_str()
            .context("IUPACName no str")?;
        let molecular_formula = properties
            .get("MolecularFormula")
            .context("No MolecularFormula")?
            .as_str()
            .context("MolecularFormula no str")?;
        let canonical_smiles = properties
            .get("CanonicalSMILES")
            .context("No CanonicalSMILES")?
            .as_str()
            .context("CanonicalSMILES no str")?;
        let molecular_weight: f32 = properties
            .get("MolecularWeight")
            .context("No MolecularWeight")?
            .as_str()
            .context("MolecularWeight no str")?
            .parse()?;

        Ok(Compound::new(
            cid,
            name.to_string(),
            iupac_name.to_string(),
            molecular_formula.to_string(),
            canonical_smiles.to_string(),
            molecular_weight,
        ))
    }
}
