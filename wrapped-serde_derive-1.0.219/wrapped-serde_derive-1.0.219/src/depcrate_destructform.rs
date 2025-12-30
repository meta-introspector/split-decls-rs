// Generated macro for StructForm (enum)
macro_rules! Depcrate_deStructForm {
() => {
// Module: crate::de
// Provides: {"StructForm"}
// Dependencies: {}
enum StructForm < 'a > { Struct , # [doc = " Contains a variant name"] ExternallyTagged (& 'a syn :: Ident) , # [doc = " Contains a variant name and an intermediate deserializer from which actual"] # [doc = " deserialization will be performed"] InternallyTagged (& 'a syn :: Ident , TokenStream) , # [doc = " Contains a variant name and an intermediate deserializer from which actual"] # [doc = " deserialization will be performed"] Untagged (& 'a syn :: Ident , TokenStream) , }
};
}
