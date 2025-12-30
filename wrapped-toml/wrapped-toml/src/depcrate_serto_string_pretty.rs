// Generated macro for to_string_pretty (function)
macro_rules! Depcrate_serto_string_pretty {
() => {
// Module: crate::ser
// Provides: {"to_string_pretty"}
// Dependencies: {}
# [doc = " Serialize the given data structure as a \"pretty\" String of TOML."] # [doc = ""] # [doc = " This is identical to `to_string` except the output string has a more"] # [doc = " \"pretty\" output. See `Serializer::pretty` for more details."] # [doc = ""] # [doc = " To serialize TOML values, instead of documents, see [`ValueSerializer`]."] # [doc = ""] # [doc = " For greater customization, instead serialize to a"] # [doc = " [`toml_edit::DocumentMut`](https://docs.rs/toml_edit/latest/toml_edit/struct.DocumentMut.html)."] # [cfg (feature = "display")] pub fn to_string_pretty < T > (value : & T) -> Result < String , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let mut output = Buffer :: new () ; let serializer = Serializer :: pretty (& mut output) ; value . serialize (serializer) ? ; Ok (output . to_string ()) }
};
}
