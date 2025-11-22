
pub struct Document {
    id: String,
    created_at: String,
    modified_at: String,
    data: Data,
}

struct Data {
    string: String,
    intent: String,
}
