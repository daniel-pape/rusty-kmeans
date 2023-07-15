use std::cmp::min;
use proptest::prelude::*;

use kmeans::squared_distance;
use kmeans::Vector;



proptest! {
    /// Squared distance must always be non-negative.
    #[test]
    fn test_squared_distance_is_non_negative(mut v_input in any::<Vec<f64>>(), mut w_input in any::<Vec<f64>>()) {
        prop_assume!(!v_input.is_empty());
        prop_assume!(!w_input.is_empty());

        let min_length = min(v_input.len(), w_input.len());
        v_input.truncate(min_length);
        w_input.truncate(min_length);
        let v: Vector  = Vector::new(v_input);
        let w: Vector  = Vector::new(w_input);
        assert!(squared_distance(&v,&w) >= 0.0);
    }


    /// Given any `Vector` its squared distance to itself
    /// must be zero.
    #[test]
    fn test_squared_distance_to_itself(input in any::<Vec<f64>>()) {
        prop_assume!(!input.is_empty());

        let v: Vector  = Vector::new(input);
        assert_eq!(squared_distance(&v,&v), 0.0);
    }

    /// Given any `Vector` its squared distance to the corresponding
    /// zero vector must equal the sum of squares of its entries.
    #[test]
    fn test_squared_distance_to_zero(input in any::<Vec<f64>>()) {
        prop_assume!(!input.is_empty());

        let v: Vector  = Vector::new(input);
        let zero: Vector = v.scale(0.0);
        assert_eq!(squared_distance(&v,&zero), v.entries.iter().map(|x| {x * x}).sum());
    }
}
