use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::f64;

#[derive(Debug, PartialEq)]
pub struct Vector {
    pub dimension: usize,
    pub entries: Vec<f64>,
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

fn check_dimension(v: &Vector, w: &Vector) {
    if v.dimension != w.dimension {
        panic!(
            "Attempt to add vectors of different dimensions: {} vs {}.",
            v.dimension, w.dimension
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
            dimension: self.dimension.clone(),
            entries: added_entries,
        }
    }

    fn scale(&self, factor: f64) -> Vector {
        Vector {
            dimension: self.dimension.clone(),
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
            dimension: dimension,
            entries: vec![0.0; dimension],
        }
    }

    pub fn new(entries: Vec<f64>) -> Self {
        Vector {
            dimension: entries.len(),
            entries: entries,
        }
    }
}

pub fn squared_distance(v: &Vector, w: &Vector) -> f64 {
    let difference = v.add(&w.scale(-1.0));
    difference.dot(&difference)
}
