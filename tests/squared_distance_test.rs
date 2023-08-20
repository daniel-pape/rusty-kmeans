use proptest::prelude::*;

use kmeans::vector::Vector;
mod custom_strategies;


proptest! {
    /// Squared distance must always be non-negative.
    #[test]
    fn test_squared_distance_is_non_negative((v,w) in custom_strategies::generate_vector_pairs()) {
        assert!(Vector::squared_distance(&v,&w) >= 0.0);
    }

    /// Given any `Vector` its squared distance to itself
    /// must be zero.
    #[test]
    fn test_squared_distance_to_itself(v in custom_strategies::generate_vectors()) {
        assert_eq!(Vector::squared_distance(&v,&v), 0.0);
    }

    /// Given any `Vector` its squared distance to the corresponding
    /// zero vector must equal the sum of squares of its entries.
    #[test]
    fn test_squared_distance_to_zero(v in custom_strategies::generate_vectors()) {
        let zero: Vector = Vector::zeros(v.size);
        let squared_sum = v.entries.iter().map(|x| {x * x}).sum::<f64>();
        assert_eq!(Vector::squared_distance(&v,&zero), squared_sum );
    }
}
