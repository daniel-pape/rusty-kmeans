use std::collections::HashMap;
use std::{f64, vec};

pub mod vector;

use vector::Vector;

type VectorId = u8;
type ClusterId = u8;

#[derive(Debug, PartialEq)]
pub struct Centroid {
    pub centroid_id: u8,
    pub value: Vector,
}

pub struct Clustering {
    k: u8,
    centroids: Vec<Centroid>,
    assignment: HashMap<VectorId, ClusterId>,
}

/// Find the centroid nearest to `v` in terms of (standard) distance.
pub fn find_nearest_centroid<'a>(
    v: &'a Vector,
    centroids: &'a Vec<Centroid>,
) -> Option<&'a Centroid> {
    centroids.iter().min_by(|first_centroid, second_centorid| {
        let x = Vector::squared_distance(&first_centroid.value, v);
        let y = Vector::squared_distance(&second_centorid.value, v);

        x.total_cmp(&y)
    })
}

impl Clustering {
    pub fn update_assignment(&mut self, dataset: HashMap<VectorId, Vector>) {
        for (vector_id, vector) in dataset {
            let nearest_centroid = find_nearest_centroid(&vector, &self.centroids).unwrap();

            self.assignment
                .insert(vector_id, nearest_centroid.centroid_id);
        }
    }

    pub fn get_assigned_vector_ids(&self, cluster_id: u8) -> Vec<u8> {
        self.assignment
            .iter()
            .filter(|(_, a_cluster_id)| **a_cluster_id == cluster_id)
            .map(|(vector_id, _)| *vector_id)
            .collect::<Vec<u8>>()
    }

    pub fn update_centroids(&mut self, dataset: &HashMap<VectorId, Vector>) {
        let mut updated_centroids: Vec<Centroid> = Vec::with_capacity(self.k as usize);

        for current_cluster_id in 0..self.k {
            let current_cluster = self
                .get_assigned_vector_ids(current_cluster_id)
                .iter()
                .map(|vector_id| &dataset[vector_id])
                .collect::<Vec<&Vector>>();

            let a = Vector::compute_average(&current_cluster);
            let x = Centroid {
                centroid_id: current_cluster_id,
                value: a,
            };

            updated_centroids.push(x);
        }

        self.centroids = updated_centroids;
    }
}

// pub fn run(k: u8, dataset: HashMap<VectorId, Vector>) {
//     let clustering = Clustering {
//         k: k,
//         assignment: HashMap::new(),
//         centroids: vec![],
//     };

//     let is_divergent = true;

//     while is_divergent {
//         clustering.update_assignment(dataset);
//         clustering.update_centroids(&dataset)
//     }
// }
