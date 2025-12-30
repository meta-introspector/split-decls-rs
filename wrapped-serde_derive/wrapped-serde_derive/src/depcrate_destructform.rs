// Generated macro for StructForm (enum)
macro_rules! Depcrate_deStructForm {
() => {
// Module: crate::de
// Provides: {"StructForm"}
// Dependencies: {}
enum StructForm < 'a > { Struct , # [doc = " Contains a variant name"] ExternallyTagged (& 'a syn :: Ident) , # [doc = " Contains a variant name"] InternallyTagged (& 'a syn :: Ident) , # [doc = " Contains a variant name"] Untagged (& 'a syn :: Ident) , }
};
}
