use proptest::prelude::*;

use kmeans::Vector;



pub fn generate_vector(dimension: usize) -> impl Strategy<Value = Vector> {
    prop::collection::vec(any::<f64>(), dimension as usize).prop_map(move |v| Vector {
        dimension: dimension,
        entries: v,
    })
}

prop_compose! {
    pub fn generate_vectors()(d in any::<u8>())(v in generate_vector(d as usize)) -> Vector { v }
}

prop_compose! {
    pub fn generate_vector_pairs()(d in any::<u8>())(v in generate_vector(d as usize), w in generate_vector(d as usize)) -> (Vector, Vector) { (v,w) }
}

prop_compose! {
    pub fn generate_vector_triples()(d in any::<u8>())(u in generate_vector(d as usize), v in generate_vector(d as usize), w in generate_vector(d as usize)) -> (Vector, Vector, Vector) { (u, v,w) }
}
