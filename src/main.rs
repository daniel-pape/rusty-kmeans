extern crate csv;

use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use kmeans::Clustering;
use kmeans::reader::{CsvReader, DataSet};
use kmeans::writer::Writer;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "k", long = "num_clusters")]
    k: usize,
    #[structopt(short = "i", long = "input_csv_file")]
    input_csv_file: String,
    #[structopt(short = "o", long = "output_dir")]
    output_dir: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    let input_csv_path = PathBuf::from_str(&options.input_csv_file)?;
    let output_dir_path = PathBuf::from_str(&options.output_dir)?;

    let dataset: DataSet = DataSet::read(input_csv_path)?;
    let clustering = Clustering::compute(options.k, dataset, 0.2);

    clustering.write(output_dir_path)?;

    Ok(())
}
