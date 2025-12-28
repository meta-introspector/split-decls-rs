macro_rules! deps {
    () => {
        Decoder!();
    };
}

macro_rules! Decodable {
    () => {
        deps!();
        # [doc = " Trait for types that can be deserialized"] # [doc = ""] # [doc = " This can be implemented using the `Decodable`, `TyDecodable` and"] # [doc = " `MetadataDecodable` macros."] # [doc = ""] # [doc = " * `Decodable` should be used in crates that don't depend on"] # [doc = "   `rustc_middle`."] # [doc = " * `MetadataDecodable` is used in `rustc_metadata` for types that contain"] # [doc = "   `rustc_metadata::rmeta::Lazy`."] # [doc = " * `TyDecodable` should be used for types that are only serialized in crate"] # [doc = "   metadata or the incremental cache. This is most types in `rustc_middle`."] pub trait Decodable < D : Decoder > : Sized { fn decode (d : & mut D) -> Self ; }
    };
}

Decodable!()