use kmeans::Vector;

#[test]
fn test_entries() {
    let v: Vector = Vector::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(v.entries, vec![1.0, 2.0, 3.0])
}


