pub mod recipe;
pub mod todo;

use mongodb::bson::oid::ObjectId;

/// Serialize ObjectId to the HTTP/JSON response as a plain hex string
/// (e.g. "507f1f77bcf86cd799439011") instead of {"$oid": "..."},
/// so the frontend can pass it straight back to e.g. DELETE /recipes/{id}.
pub fn serialize_oid_as_hex<S>(id: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match id {
        Some(oid) => s.serialize_str(&oid.to_hex()),
        None => s.serialize_none(),
    }
}
