use serde::ser::{Serialize, Serializer, SerializeStruct};

use crate::vector::Vector;

#[derive(Clone, Debug, PartialEq)]
pub struct Centroid {
    pub centroid_id: u8,
    pub value: Vector,
}

impl Serialize for Centroid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Centroid", 2)?;
        if let Err(error) = s.serialize_field("centroid_id", &self.centroid_id) {
            panic!("{}", error)
        }
        if let Err(error) = s.serialize_field("value", &self.value.entries) {
            eprint!("{}", error);
            eprint!("{:?}", &self.value.entries);
            panic!("{}", error)
        };

        s.end()
    }
}
