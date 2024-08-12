use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Serializer};

pub fn serialize_object_id_vec_as_hex_string<S: Serializer>(
    val: &Vec<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let strings: Vec<String> = val.iter().map(|id| id.to_hex()).collect();
    strings.serialize(serializer)
}
