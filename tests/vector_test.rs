#[macro_use]
extern crate approx;

mod custom_strategies;

use proptest::prelude::*;
use kmeans::vector::check_dimension;
use kmeans::vector::Operations;
use kmeans::vector::Vector;

#[test]
#[should_panic]
fn test_check_dimension() {
    check_dimension(&Vector::zeros(1), &Vector::zeros(2));
}

#[test]
fn test_entries() {
    let v: Vector = Vector::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(v.entries, vec![1.0, 2.0, 3.0])
}

#[test]
fn test_zeros() {
    assert_eq!(Vector::zeros(3).entries, vec![0.0, 0.0, 0.0])
}

proptest! {
    #[test]
    fn test_add_zero_element(v in custom_strategies::generate_vectors()) {
        let zero = Vector::zeros(v.size);

        assert_relative_eq!(v.add(&zero), v);
    }

    #[test]
    fn test_add_inverse_element(v in custom_strategies::generate_vectors()) {
        let inverse = v.scale(-1.0);
        let zero = Vector::zeros(v.size);

        assert_abs_diff_eq!(v.add(&inverse), zero);
    }

    #[test]
    fn test_add_commutative((v, w) in custom_strategies::generate_vector_pairs()) {
        assert_abs_diff_eq!(v.add(&w), w.add(&v));
    }
}
