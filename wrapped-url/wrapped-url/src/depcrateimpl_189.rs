// Generated macro for impl_189 (impl)
macro_rules! Depcrateimpl_189 {
() => {
// Module: crate
// Provides: {"impl_189"}
// Dependencies: {}
# [doc = " Deserializes this URL from a `serde` stream."] # [doc = ""] # [doc = " This implementation is only available if the `serde` Cargo feature is enabled."] # [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for Url { fn deserialize < D > (deserializer : D) -> Result < Url , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: de :: { Error , Visitor } ; struct UrlVisitor ; impl Visitor < '_ > for UrlVisitor { type Value = Url ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a string representing an URL") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : Error , { Url :: parse (s) . map_err (| err | Error :: custom (format ! ("{err}: {s:?}"))) } } deserializer . deserialize_str (UrlVisitor) } }
};
}
