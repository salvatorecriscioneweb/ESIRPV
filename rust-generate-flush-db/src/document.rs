#[derive(Debug)]
pub struct Document {
    pub id: String,
    pub file: String,
    pub hash: String,
    pub timestamp: String,
}

impl Document {
    pub fn clone(self) -> Self {
        Self {
            id: self.id,
            file: self.file,
            hash: self.hash,
            timestamp: self.timestamp,
        }
    }
}
