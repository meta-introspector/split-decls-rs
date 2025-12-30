// Generated macro for Configuration (trait)
macro_rules! Depcrate_internedConfiguration {
() => {
// Module: crate::interned
// Provides: {"Configuration"}
// Dependencies: {}
# [doc = " Trait that defines the key properties of an interned struct."] # [doc = ""] # [doc = " Implemented by the `#[salsa::interned]` macro when applied to"] # [doc = " a struct."] pub trait Configuration : Sized + 'static { const LOCATION : crate :: ingredient :: Location ; const DEBUG_NAME : & 'static str ; # [doc = " Whether this struct should be persisted with the database."] const PERSIST : bool ; # [cfg (test)] const REVISIONS : NonZeroUsize = NonZeroUsize :: new (3) . unwrap () ; # [cfg (not (test))] const REVISIONS : NonZeroUsize = NonZeroUsize :: new (1) . unwrap () ; # [doc = " The fields of the struct being interned."] type Fields < 'db > : InternedData ; # [doc = " The end user struct"] type Struct < 'db > : Copy + FromId + AsId ; # [doc = " Returns the size of any heap allocations in the output value, in bytes."] fn heap_size (_value : & Self :: Fields < '_ >) -> Option < usize > { None } # [doc = " Serialize the fields using `serde`."] # [doc = ""] # [doc = " Panics if the value is not persistable, i.e. `Configuration::PERSIST` is `false`."] fn serialize < S > (value : & Self :: Fields < '_ > , serializer : S) -> Result < S :: Ok , S :: Error > where S : plumbing :: serde :: Serializer ; # [doc = " Deserialize the fields using `serde`."] # [doc = ""] # [doc = " Panics if the value is not persistable, i.e. `Configuration::PERSIST` is `false`."] fn deserialize < 'de , D > (deserializer : D) -> Result < Self :: Fields < 'static > , D :: Error > where D : plumbing :: serde :: Deserializer < 'de > ; }
};
}
