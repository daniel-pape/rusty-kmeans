use std::{collections::HashMap, error::Error};
use std::fs::File;
use std::path::PathBuf;

use csv::ReaderBuilder;

use crate::vector::Vector;

pub type DataSet = HashMap<u8, Vector>;

pub trait CsvReader {
    fn read(path_to_file: PathBuf) -> Result<DataSet, Box<dyn Error>>;
}

impl CsvReader for DataSet {
    fn read(path_to_file: PathBuf) -> Result<DataSet, Box<dyn Error>> {
        let csv_file = File::open(path_to_file)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_file);
        let enumerated_results = reader.records().into_iter().enumerate();
        let mut dataset: HashMap<u8, Vector> = HashMap::new();

        for (row_idx, result) in enumerated_results {
            let record = result?;
            let len_record = record.len();
            let mut entries = Vec::<f64>::with_capacity(len_record);

            for idx in 0..len_record {
                let entry = record.get(idx).unwrap().parse::<f64>().unwrap();
                entries.push(entry);
            }

            let v = Vector {
                dimension: len_record,
                entries,
            };

            dataset.insert(row_idx as u8, v);
        }

        Ok(dataset)
    }
}
