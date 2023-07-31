use kmeans::vector::Vector;
use kmeans::{find_nearest_centroid, Centroid};

#[test]
fn test_find_nearest_centroid() {
    let v = Vector::zeros(1);
    let centroids = &vec![];
    let nearest_centroid = find_nearest_centroid(&v, centroids);

    assert!(nearest_centroid.is_none());
}

#[test]
fn test_find_nearest_centroid_v2() {
    let v = Vector::zeros(1);

    let centroids = vec![
        Centroid {
            centroid_id: 1,
            value: Vector::new(vec![1f64]),
        },
        Centroid {
            centroid_id: 2,
            value: Vector::new(vec![2f64]),
        },
        Centroid {
            centroid_id: 3,
            value: Vector::new(vec![3f64]),
        },
    ];

    let nearest_centroid = find_nearest_centroid(&v, &centroids);

    assert!(nearest_centroid.is_some());
    assert_eq!(nearest_centroid.unwrap().centroid_id, 1);
    assert_eq!(nearest_centroid.unwrap().value, Vector::new(vec![1f64]));
}
