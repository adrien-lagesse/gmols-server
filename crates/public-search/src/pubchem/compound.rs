use serde;

#[derive(Debug, serde::Serialize)]
pub struct Compound {
    cid: u64,
    name: String,
    iupac_name: String,
    description: Option<String>,
    molecular_formula: String,
    canonical_smiles: String,
    molecular_weight: f32,
}

impl Compound {
    pub fn new(
        cid: u64,
        name: String,
        iupac_name: String,
        description: Option<String>,
        molecular_formula: String,
        canonical_smiles: String,
        molecular_weight: f32,
    ) -> Compound {
        Compound { cid, name, iupac_name, description, molecular_formula, canonical_smiles, molecular_weight }
    }
}
