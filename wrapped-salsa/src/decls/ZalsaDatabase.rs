macro_rules! deps {
    () => {
        ZalsaLocal!();
        RawDatabase!();
        Zalsa!();
    };
}

macro_rules! ZalsaDatabase {
    () => {
        deps!();
        # [doc = " Internal plumbing trait."] # [doc = ""] # [doc = " [`ZalsaDatabase`] is created automatically when [`#[salsa::db]`](`crate::db`)"] # [doc = " is attached to a database struct. it Contains methods that give access"] # [doc = " to the internal data from the `storage` field."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The system assumes this is implemented by a salsa procedural macro"] # [doc = " which makes use of private data from the [`Storage`](`crate::storage::Storage`) struct."] # [doc = " Do not implement this yourself, instead, apply the [`#[salsa::db]`](`crate::db`) macro"] # [doc = " to your database."] pub unsafe trait ZalsaDatabase : Any { # [doc = " Plumbing method: access both zalsa and zalsa-local at once."] # [doc = " More efficient if you need both as it does only a single vtable dispatch."] # [doc (hidden)] fn zalsas (& self) -> (& Zalsa , & ZalsaLocal) { (self . zalsa () , self . zalsa_local ()) } # [doc = " Plumbing method: Access the internal salsa methods."] # [doc (hidden)] fn zalsa (& self) -> & Zalsa ; # [doc = " Plumbing method: Access the internal salsa methods for mutating the database."] # [doc = ""] # [doc = " **WARNING:** Triggers cancellation to other database handles."] # [doc = " This can lead to deadlock!"] # [doc (hidden)] fn zalsa_mut (& mut self) -> & mut Zalsa ; # [doc = " Access the thread-local state associated with this database"] # [doc (hidden)] fn zalsa_local (& self) -> & ZalsaLocal ; # [doc = " Clone the database."] # [doc (hidden)] fn fork_db (& self) -> RawDatabase < 'static > ; }
    };
}

ZalsaDatabase!();