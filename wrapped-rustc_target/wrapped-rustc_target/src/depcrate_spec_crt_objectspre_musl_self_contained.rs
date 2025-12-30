// Generated macro for pre_musl_self_contained (function)
macro_rules! Depcrate_spec_crt_objectspre_musl_self_contained {
() => {
// Module: crate::spec::crt_objects
// Provides: {"pre_musl_self_contained"}
// Dependencies: {}
pub (super) fn pre_musl_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crt1.o" , "crti.o" , "crtbegin.o"]) , (LinkOutputKind :: DynamicPicExe , & ["Scrt1.o" , "crti.o" , "crtbeginS.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crt1.o" , "crti.o" , "crtbegin.o"]) , (LinkOutputKind :: StaticPicExe , & ["rcrt1.o" , "crti.o" , "crtbeginS.o"]) , (LinkOutputKind :: DynamicDylib , & ["crti.o" , "crtbeginS.o"]) , (LinkOutputKind :: StaticDylib , & ["crti.o" , "crtbeginS.o"]) ,]) }
};
}
