use std::f64;

pub mod vector;

use vector::Operations;
use vector::Vector;

pub fn squared_distance(v: &Vector, w: &Vector) -> f64 {
    let difference = v.add(&w.scale(-1.0));
    difference.dot(&difference)
}
