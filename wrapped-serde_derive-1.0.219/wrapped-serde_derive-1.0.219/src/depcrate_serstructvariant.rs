// Generated macro for StructVariant (enum)
macro_rules! Depcrate_serStructVariant {
() => {
// Module: crate::ser
// Provides: {"StructVariant"}
// Dependencies: {}
enum StructVariant < 'a > { ExternallyTagged { variant_index : u32 , variant_name : & 'a Name , } , InternallyTagged { tag : & 'a str , variant_name : & 'a Name , } , Untagged , }
};
}
