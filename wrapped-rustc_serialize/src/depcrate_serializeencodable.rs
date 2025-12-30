// Generated macro for Encodable (trait)
macro_rules! Depcrate_serializeEncodable {
() => {
// Module: crate::serialize
// Provides: {"Encodable"}
// Dependencies: {}
# [doc = " Trait for types that can be serialized"] # [doc = ""] # [doc = " This can be implemented using the `Encodable`, `TyEncodable` and"] # [doc = " `MetadataEncodable` macros."] # [doc = ""] # [doc = " * `Encodable` should be used in crates that don't depend on"] # [doc = "   `rustc_middle`."] # [doc = " * `MetadataEncodable` is used in `rustc_metadata` for types that contain"] # [doc = "   `rustc_metadata::rmeta::Lazy`."] # [doc = " * `TyEncodable` should be used for types that are only serialized in crate"] # [doc = "   metadata or the incremental cache. This is most types in `rustc_middle`."] pub trait Encodable < S : Encoder > : PointeeSized { fn encode (& self , s : & mut S) ; }
};
}
