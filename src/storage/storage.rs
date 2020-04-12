/// Trait for storing favorites on a storage.
pub trait Storage {
    /// Initization. Any required pre-storage functions must be added here.
    fn open(&self);

    /// Save the favourite in the storage.
    fn save(&self);

    /// Return the original favourite identification.
    fn get_id(&self) -> &String;
}

