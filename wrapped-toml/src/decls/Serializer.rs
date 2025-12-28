macro_rules! deps {
    () => {
        Style!();
        Table!();
        Buffer!();
    };
}

macro_rules! Serializer {
    () => {
        deps!();
        # [doc = " Serialization for TOML documents."] # [doc = ""] # [doc = " This structure implements serialization support for TOML to serialize an"] # [doc = " arbitrary type to TOML. Note that the TOML format does not support all"] # [doc = " datatypes in Rust, such as enums, tuples, and tuple structs. These types"] # [doc = " will generate an error when serialized."] # [doc = ""] # [doc = " Currently a serializer always writes its output to an in-memory `String`,"] # [doc = " which is passed in when creating the serializer itself."] # [doc = ""] # [doc = " To serialize TOML values, instead of documents, see"] # [doc = " [`ValueSerializer`][super::value::ValueSerializer]."] pub struct Serializer < 'd > { buf : & 'd mut Buffer , style : style :: Style , table : Table , }
    };
}

Serializer!();