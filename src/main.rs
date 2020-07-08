use csv;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

const HORVATH: &[u8; 9577] = include_bytes!("../coefficients/CpG-coefficients.csv");
const PHENO: &[u8; 12524] = include_bytes!("../coefficients/coef_pheno.csv");

struct Model {
    name: String,
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
        name: String::from("Horvath"),
        coefs: load_coefficients(HORVATH)?,
        adjust: |m| m.exp() * 21.0 - 1.0,
    };

    let p = Model {
        name: String::from("Phenotype"),
        coefs: load_coefficients(PHENO)?,
        adjust: std::convert::identity,
    };

    let models = vec![&h, &p];

    let ages = apply(&models, &config.file)?;

    for (&model, age) in models.iter().zip(ages) {
        println!("{:9} : {:.2} years", model.name, age);
    }

    Ok(())
}

fn apply(models: &[&Model], file: &PathBuf) -> Result<Vec<f32>, AppErr> {
    let mut rdr = csv::Reader::from_path(file)?;

    let mut ages: Vec<f32> = Vec::with_capacity(models.len());

    for model in models {
        ages.push(*model.coefs.get("intercept").expect("intercept not found"));
    }

    for result in rdr.records() {
        let record = result?;
        let key = record[0].to_owned();
        let value: f32 = record[1].parse().unwrap_or_default();

        for (&model, age) in models.iter().zip(ages.iter_mut()) {
            model
                .coefs
                .get(&key)
                .iter()
                .for_each(|&w| *age += w * value);
        }
    }

    for (model, age) in models.iter().zip(ages.iter_mut()) {
        *age = (model.adjust)(*age)
    }
    Ok(ages)
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

type AppErr = Box<dyn std::error::Error>;