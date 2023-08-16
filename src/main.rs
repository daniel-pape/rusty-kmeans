extern crate csv;

use std::collections::HashMap;
use std::env;
use std::error::Error;

use serde::Serialize;

use kmeans::compute;
use kmeans::writer::Writer;
use kmeans::vector::Vector;

#[derive(Debug, Serialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let clustering = compute(
        3,
        HashMap::from([
            (1, Vector::new(vec![0.0])),
            (2, Vector::new(vec![1.0])),
            (3, Vector::new(vec![2.0])),
        ]),
        0.2,
    );

    let output_dir_path = env::current_dir()?.as_path().join("output");
    match clustering.write(output_dir_path) {
        Ok(_) => {}
        Err(error) => panic!("{}", error)
    }

    Ok(())
}
