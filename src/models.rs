use super::AppErr;
use std::collections::HashMap;

const HORVATH: &[u8; 9577] = include_bytes!("../coefficients/CpG-coefficients.csv");
const PHENO: &[u8; 12524] = include_bytes!("../coefficients/coef_pheno.csv");

pub struct Model {
    pub name: String,
    coefs: HashMap<String, f32>,
    adjust: fn(f32) -> f32,
}

impl Model {
    pub fn intercept(&self) -> Option<&f32> {
        self.coefs.get("intercept")
    }

    pub fn weight(&self, probe_id: &str) -> f32 {
        match self.coefs.get(probe_id) {
            None => 0.0,
            Some(&v) => v,
        }
    }

    pub fn adjustment(&self, age: f32) -> f32 {
        (self.adjust)(age)
    }

    //
    // Models
    //

    pub fn horvath() -> Result<Model, AppErr> {
        load_coefficients(HORVATH).map(|bytes| Model {
            name: String::from("Horvath Clock"),
            coefs: bytes,
            adjust: |x| match x < 0.0 {
                true => 21.0 * x.exp() - 1.0,
                false => 21.0 * x + 21.0,
            },
        })
    }

    pub fn pheno() -> Result<Model, AppErr> {
        load_coefficients(PHENO).map(|bytes| Model {
            name: String::from("DNAm PhenoAge"),
            coefs: bytes,
            adjust: std::convert::identity,
        })
    }

    pub fn all() -> Result<Vec<Model>, AppErr> {
        Ok(vec![Model::horvath()?, Model::pheno()?])
    }
}

fn load_coefficients(source: &[u8]) -> Result<HashMap<String, f32>, AppErr> {
    let mut rdr = csv::ReaderBuilder::new().from_reader(source);

    let mut cs: HashMap<String, f32> = HashMap::with_capacity(source.len());

    for result in rdr.records() {
        let record = result?;
        let key = record[0].to_owned();
        assert!(cs.contains_key(&key) == false);
        let value = record[1].parse()?;
        cs.insert(key, value);
    }

    Ok(cs)
}
