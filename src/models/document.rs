use crate::models::data::Data;

pub struct Document {
    id: String,
    created_at: String,
    modified_at: String,
    data: Data,
}
