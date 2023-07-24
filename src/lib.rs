use std::collections::HashMap;
use std::f64;

pub mod vector;

use vector::Vector;

type VectorId = u8;
type ClusterId = u8;
pub struct Centroid {
    centroid_id: u8,
    value: Vector,
}

pub struct Clustering {
    k: u8,
    centroids: Vec<Centroid>,
    assignment: HashMap<VectorId, ClusterId>,
}

pub fn compute_distances<'a>(
    v: &'a Vector,
    centroids: &'a Vec<Centroid>,
) -> Vec<(f64, &'a Centroid)> {
    return centroids
        .iter()
        .map(|c| (Vector::squared_distance(v, &c.value), c))
        .collect();
}

pub fn find_nearest_centroid<'a>(
    v: &'a Vector,
    centroids: &'a Vec<Centroid>,
) -> Option<&'a Centroid> {
    // test: should only be None iff centroids is empty
    compute_distances(v, &centroids)
        .iter()
        .min_by(|x, y: &&(f64, &Centroid)| x.0.total_cmp(&y.0))
        .map(|x| x.1)
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

            updated_centroids.push(Centroid {
                centroid_id: current_cluster_id,
                value: Vector::compute_average(current_cluster),
            })
        }

        self.centroids = updated_centroids;
    }
}
