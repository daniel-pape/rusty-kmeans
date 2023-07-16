use proptest::prelude::*;
use core::cmp::min;
use kmeans::Vector;

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
    fn test_add_zero_element(mut v_input in any::<Vec<f64>>()) {
        let zero = Vector::zeros(v_input.len());
        let v = Vector::new(v_input);

        assert_eq!(v.add(&zero).entries, v.entries);
    }

    #[test]
    fn test_add_commutative(mut v_input in any::<Vec<f64>>(), mut w_input in any::<Vec<f64>>()) {
        prop_assume!(!v_input.is_empty());
        prop_assume!(!w_input.is_empty());

        let min_length = min(v_input.len(), w_input.len());
        v_input.truncate(min_length);
        w_input.truncate(min_length);

        let v = Vector::new(v_input);
        let w = Vector::new(w_input);

        assert_eq!(v.add(&w).entries, w.add(&v).entries);
    }
}
