// Generated macro for Accuracy (struct)
macro_rules! DepcrateAccuracy {
() => {
// Module: crate
// Provides: {"Accuracy"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " Accuracy ::= SEQUENCE {"] # [doc = "     seconds        INTEGER              OPTIONAL,"] # [doc = "     millis     [0] INTEGER  (1..999)    OPTIONAL,"] # [doc = "     micros     [1] INTEGER  (1..999)    OPTIONAL  }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] pub struct Accuracy { # [asn1 (optional = "true")] pub seconds : Option < u64 > , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub millis : Option < i16 > , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , optional = "true")] pub micros : Option < i16 > , }
};
}
