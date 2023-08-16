use std::path::PathBuf;
use std::io::Error;

pub trait Writer {
    fn write(&self, output_path: PathBuf) -> Result<(), Error>;
}

pub trait ClusteringWriter {
    fn write_centroids(&self, output_path: PathBuf) -> Result<(), Error>;
    fn write_cluster_assignment(&self, output_path: PathBuf) -> Result<(), Error>;
}

impl<T: ClusteringWriter> Writer for T {
    fn write(&self, output_dir: PathBuf) -> Result<(), Error> {
        let centroids_file_path = output_dir.join("centroids.csv");
        let cluster_assignment_file_path = output_dir.join("cluster_assignment.csv");

        if let Err(error) = self.write_centroids(centroids_file_path) {
            return Err(error);
        }

        if let Err(error) = self.write_cluster_assignment(cluster_assignment_file_path) {
            return Err(error);
        }


        Ok(())
    }
}


