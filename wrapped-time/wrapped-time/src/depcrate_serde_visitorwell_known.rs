// Generated macro for well_known (macro)
macro_rules! Depcrate_serde_visitorwell_known {
() => {
// Module: crate::serde::visitor
// Provides: {"well_known"}
// Dependencies: {}
# [doc = " Implement a visitor for a well-known format."] macro_rules ! well_known { ($ article : literal , $ name : literal , $ ($ ty : tt) +) => { # [cfg (feature = "parsing")] impl de :: Visitor <'_ > for Visitor <$ ($ ty) +> { type Value = OffsetDateTime ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter <'_ >) -> fmt :: Result { formatter . write_str (concat ! ($ article , " " , $ name , "-formatted `OffsetDateTime`")) } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < OffsetDateTime , E > { OffsetDateTime :: parse (value , &$ ($ ty) +) . map_err (E :: custom) } } # [cfg (feature = "parsing")] impl <'a > de :: Visitor <'a > for Visitor < Option <$ ($ ty) +>> { type Value = Option < OffsetDateTime >; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter <'_ >) -> fmt :: Result { formatter . write_str (concat ! ($ article , " " , $ name , "-formatted `Option<OffsetDateTime>`")) } # [inline] fn visit_some < D : Deserializer <'a >> (self , deserializer : D ,) -> Result < Option < OffsetDateTime >, D :: Error > { deserializer . deserialize_any (Visitor ::<$ ($ ty) +> (PhantomData)) . map (Some) } # [inline] fn visit_none < E : de :: Error > (self) -> Result < Option < OffsetDateTime >, E > { Ok (None) } # [inline] fn visit_unit < E : de :: Error > (self) -> Result < Self :: Value , E > { Ok (None) } } } ; }
};
}
