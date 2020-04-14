use crate::storage::data::Data;

/// Trait for storing favorites on a storage.
pub trait Storage {
    /// Save the favourite in the storage.
    fn save(&self, record: &Data);
}
