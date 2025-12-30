// Generated macro for deserialize_enum (macro)
macro_rules! Depcrate_de_implsdeserialize_enum {
() => {
// Module: crate::de::impls
// Provides: {"deserialize_enum"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] macro_rules ! deserialize_enum { ($ name : ident $ name_kind : ident ($ ($ variant : ident ; $ bytes : expr ; $ index : expr) ,*) $ expecting_message : expr , $ deserializer : expr) => { variant_identifier ! { $ name_kind ($ ($ variant ; $ bytes ; $ index) ,*) $ expecting_message , VARIANTS } struct EnumVisitor ; impl <'de > Visitor <'de > for EnumVisitor { type Value = $ name ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (concat ! ("a " , stringify ! ($ name))) } fn visit_enum < A > (self , data : A) -> Result < Self :: Value , A :: Error > where A : EnumAccess <'de >, { match tri ! (data . variant ()) { $ (($ name_kind :: $ variant , v) => v . newtype_variant () . map ($ name :: $ variant) ,) * } } } $ deserializer . deserialize_enum (stringify ! ($ name) , VARIANTS , EnumVisitor) } }
};
}
