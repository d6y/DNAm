use csv;
use std::path::PathBuf;
use structopt::StructOpt;

mod models;
use models::Model;

#[derive(StructOpt, Debug)]
struct Config {
    /// Beta values
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), AppErr> {
    let config = Config::from_args();

    let models = Model::all()?;
    let ages = apply(&models, &config.file)?;

    for (model, age) in models.iter().zip(ages) {
        println!("{:9} : {:.2} years", model.name, age);
    }

    Ok(())
}

fn apply(models: &[Model], file: &PathBuf) -> Result<Vec<f32>, AppErr> {
    let mut rdr = csv::Reader::from_path(file)?;

    let mut ages: Vec<f32> = Vec::with_capacity(models.len());

    for model in models {
        ages.push(*model.intercept().expect("intercept not found"));
    }

    for result in rdr.records() {
        let record = result?;
        let key = record[0].to_owned();
        let value: f32 = record[1].parse().unwrap_or_default();

        for (model, age) in models.iter().zip(ages.iter_mut()) {
            *age += value * model.weight(&key);
        }
    }

    for (model, age) in models.iter().zip(ages.iter_mut()) {
        *age = model.adjustment(*age)
    }
    Ok(ages)
}

type AppErr = Box<dyn std::error::Error>;
