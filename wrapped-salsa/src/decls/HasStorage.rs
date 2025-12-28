macro_rules! deps {
    () => {
        Database!();
        Storage!();
    };
}

macro_rules! HasStorage {
    () => {
        deps!();
        # [doc = " Access the \"storage\" of a Salsa database: this is an internal plumbing trait"] # [doc = " automatically implemented by `#[salsa::db]` applied to a struct."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `storage` and `storage_mut` fields must both return a reference to the same"] # [doc = " storage field which must be owned by `self`."] pub unsafe trait HasStorage : Database + Clone + Sized { fn storage (& self) -> & Storage < Self > ; fn storage_mut (& mut self) -> & mut Storage < Self > ; }
    };
}

HasStorage!();