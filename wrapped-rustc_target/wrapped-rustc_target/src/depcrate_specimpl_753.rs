// Generated macro for impl_753 (impl)
macro_rules! Depcrate_specimpl_753 {
() => {
// Module: crate::spec
// Provides: {"impl_753"}
// Dependencies: {}
impl LinkOutputKind { pub fn can_link_dylib (self) -> bool { match self { LinkOutputKind :: StaticNoPicExe | LinkOutputKind :: StaticPicExe => false , LinkOutputKind :: DynamicNoPicExe | LinkOutputKind :: DynamicPicExe | LinkOutputKind :: DynamicDylib | LinkOutputKind :: StaticDylib | LinkOutputKind :: WasiReactorExe => true , } } }
};
}
