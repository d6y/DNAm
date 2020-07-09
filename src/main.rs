use csv;
use std::path::PathBuf;
use structopt::StructOpt;

mod models;
use models::Model;

#[derive(StructOpt, Debug)]
struct Config {
    /// Treat CSV values as M-values (default: values are beta)
    #[structopt(short, long)]
    m_values: bool,

    /// Methylation values (CSV format: column 1 is probe ID, column 2 is reading)
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), AppErr> {
    let config = Config::from_args();

    let convert: fn(f32) -> f32 = if config.m_values {
        // Experimental, may be wrong: Convert m-values to beta values
        |m| {
            let p = 2f32.powf(m);
            let b = p / (p + 1.0);
            assert!(b >= 0.0);
            b
        }
    } else {
        std::convert::identity
    };

    let models = Model::all()?;
    let ages = apply(&models, &config.file, convert)?;

    for (model, age) in models.iter().zip(ages) {
        println!("{:9} : {:.2} years", model.name, age);
    }

    Ok(())
}

fn apply(models: &[Model], file: &PathBuf, convert: fn(f32) -> f32) -> Result<Vec<f32>, AppErr> {
    let mut rdr = csv::Reader::from_path(file)?;

    let mut ages: Vec<f32> = Vec::with_capacity(models.len());

    for model in models {
        ages.push(*model.intercept().expect("models must have an intercept"));
    }

    for result in rdr.records() {
        let record = result?;
        let probe_id = &record[0];

        let subject_index = 1; // Consider supporting multiple subjects in the future
        let subject_value: f32 = record[subject_index]
            .parse()
            .map(convert)
            .unwrap_or_default();

        for (model, age) in models.iter().zip(ages.iter_mut()) {
            *age += subject_value * model.weight(probe_id);
        }
    }

    for (model, age) in models.iter().zip(ages.iter_mut()) {
        *age = model.adjustment(*age)
    }
    Ok(ages)
}

type AppErr = Box<dyn std::error::Error>;
