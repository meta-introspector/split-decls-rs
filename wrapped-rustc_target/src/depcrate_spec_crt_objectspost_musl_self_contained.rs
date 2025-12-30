// Generated macro for post_musl_self_contained (function)
macro_rules! Depcrate_spec_crt_objectspost_musl_self_contained {
() => {
// Module: crate::spec::crt_objects
// Provides: {"post_musl_self_contained"}
// Dependencies: {}
pub (super) fn post_musl_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crtend.o" , "crtn.o"]) , (LinkOutputKind :: DynamicPicExe , & ["crtendS.o" , "crtn.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crtend.o" , "crtn.o"]) , (LinkOutputKind :: StaticPicExe , & ["crtendS.o" , "crtn.o"]) , (LinkOutputKind :: DynamicDylib , & ["crtendS.o" , "crtn.o"]) , (LinkOutputKind :: StaticDylib , & ["crtendS.o" , "crtn.o"]) ,]) }
};
}
