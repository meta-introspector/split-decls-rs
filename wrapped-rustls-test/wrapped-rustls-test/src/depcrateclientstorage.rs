// Generated macro for ClientStorage (struct)
macro_rules! DepcrateClientStorage {
() => {
// Module: crate
// Provides: {"ClientStorage"}
// Dependencies: {}
pub struct ClientStorage { storage : Arc < dyn rustls :: client :: ClientSessionStore > , ops : Mutex < Vec < ClientStorageOp > > , alter_max_early_data_size : Option < (u32 , u32) > , }
};
}
