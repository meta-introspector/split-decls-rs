// Generated macro for all (function)
macro_rules! Depcrate_spec_crt_objectsall {
() => {
// Module: crate::spec::crt_objects
// Provides: {"all"}
// Dependencies: {}
pub (super) fn all (obj : & 'static str) -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & [obj]) , (LinkOutputKind :: DynamicPicExe , & [obj]) , (LinkOutputKind :: StaticNoPicExe , & [obj]) , (LinkOutputKind :: StaticPicExe , & [obj]) , (LinkOutputKind :: DynamicDylib , & [obj]) , (LinkOutputKind :: StaticDylib , & [obj]) ,]) }
};
}
