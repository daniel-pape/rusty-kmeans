
#[derive(Debug)]
pub struct Vector {
    pub dimension: usize,
    pub entries: Vec<f64>,
}

fn check_dimension(v: &Vector, w: &Vector) {
    if v.dimension != w.dimension {
        panic!("Attempt to add vectors of different dimensions: {} vs {}.", v.dimension, w.dimension)
    }
}

impl Vector {
    pub fn zeros(dimension: usize) -> Self {
        Vector {
            dimension: dimension,
            entries: vec![0.0; dimension]
        }
    }


    pub fn new(entries: Vec<f64>) -> Self {
        Vector {
            dimension: entries.len(),
            entries: entries,
        }
    }

    pub fn add(&self, v: &Vector) -> Vector {
        check_dimension(self, &v);

        let added_entries: Vec<f64> = self
            .entries
            .iter()
            .zip(v.entries.iter())
            .map(|(x, y)| { x + y })
            .collect::<Vec<f64>>();

        Vector {
            dimension: self.dimension.clone(),
            entries: added_entries,
        }
    }

    pub fn scale(&self, factor: f64) -> Vector {
        Vector {
            dimension: self.dimension.clone(),
            entries: self.entries.iter().map(|x| { factor * x }).collect::<Vec<f64>>(),
        }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.entries
            .iter()
            .zip(other.entries.iter())
            .map(|(x, y)| { x * y })
            .sum()
    }
}

pub fn squared_distance(v: &Vector, w: &Vector) -> f64 {
    let difference = v.add(&w.scale(-1.0));
    difference.dot(&difference)
}
