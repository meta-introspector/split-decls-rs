// Generated macro for Tag (enum)
macro_rules! Depcrate_derTag {
() => {
// Module: crate::der
// Provides: {"Tag"}
// Dependencies: {}
# [allow (clippy :: upper_case_acronyms)] # [derive (Clone , Copy , Eq , PartialEq)] # [repr (u8)] pub (crate) enum Tag { Boolean = 0x01 , Integer = 0x02 , BitString = 0x03 , OctetString = 0x04 , OID = 0x06 , Enum = 0x0A , Sequence = CONSTRUCTED | 0x10 , UTCTime = 0x17 , GeneralizedTime = 0x18 , # [allow (clippy :: identity_op)] ContextSpecificConstructed0 = CONTEXT_SPECIFIC | CONSTRUCTED | 0 , ContextSpecificConstructed1 = CONTEXT_SPECIFIC | CONSTRUCTED | 1 , ContextSpecificConstructed3 = CONTEXT_SPECIFIC | CONSTRUCTED | 3 , }
};
}
