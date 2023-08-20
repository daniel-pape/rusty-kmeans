extern crate csv;

use std::collections::HashMap;
use std::f64;
use std::io::Error;
use std::path::PathBuf;

use csv::WriterBuilder;

pub use centroid::Centroid;
use vector::Vector;
use writer::ClusteringWriter;

pub mod vector;
pub mod centroid;
pub mod writer;
pub mod reader;

type VectorId = u8;
type ClusterId = u8;

pub struct Clustering {
    pub k: u8,
    pub centroids: Vec<Centroid>,
    pub assignment: HashMap<VectorId, ClusterId>,
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
    pub fn update_assignment(&mut self, dataset: &HashMap<VectorId, Vector>) {
        for (vector_id, vector) in dataset {
            let nearest_centroid = find_nearest_centroid(&vector, &self.centroids).unwrap();

            self.assignment
                .insert(*vector_id, nearest_centroid.centroid_id);
        }
    }

    fn get_assigned_vector_ids(&self, cluster_id: u8) -> Vec<u8> {
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
                value: Vector::compute_average(&current_cluster),
            });
        }

        self.centroids = updated_centroids;
    }

    pub fn is_convergent(&self, previous_centroids: Vec<Centroid>, eps: f64) -> bool {
        self.centroids
            .iter()
            .zip(previous_centroids.iter())
            .map(|(left, right)| Vector::squared_distance(&left.value, &right.value))
            .sum::<f64>()
            < eps
    }

    pub fn compute(k: u8, dataset: HashMap<VectorId, Vector>, eps: f64) -> Clustering {
        let mut clustering = Clustering {
            k,
            assignment: HashMap::new(),
            centroids: dataset
                .values()
                .take(k as usize)
                .enumerate()
                .map(|(i, v)| Centroid {
                    centroid_id: i as u8,
                    value: v.clone(),
                })
                .collect(),
        };

        loop {
            let previous_centroids: Vec<Centroid> = clustering.centroids.clone();

            clustering.update_assignment(&dataset);
            clustering.update_centroids(&dataset);

            if clustering.is_convergent(previous_centroids, eps) {
                println!("Clustering is convergent.");
                break;
            }
        }

        clustering
    }
}


impl ClusteringWriter for Clustering {
    fn write_centroids(&self, output_path: PathBuf) -> Result<(), Error> {
        let mut centroid_writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(output_path)?;

        for centroid in self.centroids.iter() {
            if let Err(error) = centroid_writer.serialize(centroid) {
                return Err(error.into());
            }
        }

        centroid_writer.flush()?;

        Ok(())
    }

    fn write_cluster_assignment(&self, output_path: PathBuf) -> Result<(), Error> {
        let mut cluster_assignment_writer = WriterBuilder::new()
            .has_headers(true)
            .from_path(output_path)?;

        if let Err(error) = cluster_assignment_writer.write_record(vec!["vector_id", "cluster_id"]) {
            return Err(error.into());
        };

        for (vector_id, cluster_id) in &self.assignment {
            cluster_assignment_writer.serialize((vector_id, cluster_id))?;
        }

        cluster_assignment_writer.flush()
    }
}
