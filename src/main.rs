extern crate csv;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use kmeans::Clustering;
use kmeans::reader::{CsvReader, DataSet};
use kmeans::vector::Vector;
use kmeans::writer::Writer;

fn main() -> Result<(), Box<dyn Error>> {
    let input_csv_path = PathBuf::from_str("/Users/dev/projects/rust/points/src/points/cluster_data.csv")?;
    let output_dir_path = env::current_dir()?.as_path().join("output");

    let dataset: HashMap<u8, Vector> = DataSet::read(input_csv_path)?;
    let clustering = Clustering::compute(3, dataset, 0.2);

    clustering.write(output_dir_path)?;

    Ok(())
}
