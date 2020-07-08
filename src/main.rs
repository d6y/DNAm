use csv;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

type AppErr = Box<dyn std::error::Error>;

const HORVATH: &[u8; 9577] = include_bytes!("../coefficients/CpG-coefficients.csv");

const PHENO: &[u8; 12524] = include_bytes!("../coefficients/coef_pheno.csv");

struct Model {
    coefs: HashMap<String, f32>,
    adjust: fn(f32) -> f32,
}

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), AppErr> {
    let config = Config::from_args();

    let h = Model {
        coefs: load_coefficients(HORVATH)?,
        adjust: |m| m.exp() * 21.0 - 1.0
    };

    let p = Model {
        coefs: load_coefficients(PHENO)?,
        adjust: std::convert::identity
    };

    let h_age = apply(&h, &config.file)?;
    println!("horvath:   {:.2}", h_age);

    let p_age = apply(&p, &config.file)?;
    println!("phenotype: {:.2}", p_age);

    Ok(())
}

fn apply(model: &Model, file: &PathBuf) -> Result<f32, AppErr> {
    let mut rdr = csv::Reader::from_path(file)?;

    let mut age: f32 = *model.coefs.get("intercept").expect("intercept not found");

    for result in rdr.records() {
        let record = result?;
        let key = record[0].to_owned();
        let value: f32 = if record[1].is_empty() {
            0.0
        } else {
            record[1].parse()?
        };

        model.coefs.get(&key).iter().for_each(|&w| age += w * value);
    }

    Ok((model.adjust)(age))
}

fn load_coefficients(source: &[u8]) -> Result<HashMap<String, f32>, AppErr> {
    let mut rdr = csv::ReaderBuilder::new().from_reader(source);

    let mut cs: HashMap<String, f32> = HashMap::with_capacity(source.len());

    for result in rdr.records() {
        let record = result?;
        let key = record[0].to_owned();
        let value = record[1].parse()?;
        cs.insert(key, value);
    }

    Ok(cs)
}
