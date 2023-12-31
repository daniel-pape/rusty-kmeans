use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use serde::Serializer;
use serde::ser::SerializeSeq;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    pub size: usize,
    pub entries: Vec<f64>,
}

impl Serialize for Vector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(self.size))?;

        for e in self.entries.iter() {
            seq.serialize_element::<f64>(e)?
        }

        seq.end()
    }
}

impl AbsDiffEq for Vector {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.entries
            .iter()
            .zip(other.entries.iter())
            .all(|(x, y)| f64::abs_diff_eq(x, y, epsilon))
    }
}

impl RelativeEq for Vector {
    fn default_max_relative() -> Self::Epsilon {
        1.0
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.entries
            .iter()
            .zip(other.entries.iter())
            .all(|(x, y)| f64::relative_eq(x, y, epsilon, max_relative))
    }
}

impl UlpsEq for Vector {
    fn default_max_ulps() -> u32 {
        4
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.entries
            .iter()
            .zip(other.entries.iter())
            .all(|(x, y)| f64::relative_eq(x, y, epsilon, max_ulps as f64))
    }
}

pub trait Operations {
    fn add(&self, other: &Self) -> Self;
    fn scale(&self, factor: f64) -> Self;
    fn dot(&self, other: &Vector) -> f64;
}

pub fn check_dimension(v: &Vector, w: &Vector) {
    if v.size != w.size {
        panic!(
            "Attempt to perform binary operation on vectors of different dimensions: {} vs {}.",
            v.size, w.size
        )
    }
}

impl Operations for Vector {
    fn add(&self, other: &Self) -> Self {
        check_dimension(self, &other);

        let added_entries: Vec<f64> = self
            .entries
            .iter()
            .zip(other.entries.iter())
            .map(|(x, y)| x + y)
            .collect::<Vec<f64>>();

        Vector {
            size: self.size.clone(),
            entries: added_entries,
        }
    }

    fn scale(&self, factor: f64) -> Vector {
        Vector {
            size: self.size.clone(),
            entries: self
                .entries
                .iter()
                .map(|x| factor * x)
                .collect::<Vec<f64>>(),
        }
    }

    fn dot(&self, other: &Vector) -> f64 {
        self.entries
            .iter()
            .zip(other.entries.iter())
            .map(|(x, y)| x * y)
            .sum()
    }
}

impl Vector {
    pub fn zeros(dimension: usize) -> Self {
        Vector {
            size: dimension,
            entries: vec![0.0; dimension],
        }
    }

    pub fn new(entries: Vec<f64>) -> Self {
        Vector {
            size: entries.len(),
            entries: entries,
        }
    }

    pub fn compute_average(vectors: &Vec<&Vector>) -> Vector {
        // TODO: Don't get dimension by drawing!
        let d = vectors[0].size;
        let sum = vectors.iter().fold(Vector::zeros(d), |sum, w| sum.add(w));
        let cardinality = vectors.len() as f64;
        let inverse_cardinality = 1.0 / cardinality;

        sum.scale(inverse_cardinality)
    }

    pub fn squared_distance(v: &Vector, w: &Vector) -> f64 {
        let difference = v.add(&w.scale(-1.0));
        difference.dot(&difference)
    }

    pub fn compute_squared_distancesa(v: &Vector, centroids: &Vec<&Vector>) -> Vec<f64> {
        return centroids
            .iter()
            .map(|w| (Vector::squared_distance(v, w)))
            .collect();
    }
}
