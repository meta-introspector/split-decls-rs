// Generated macro for pre_mingw_self_contained (function)
macro_rules! Depcrate_spec_crt_objectspre_mingw_self_contained {
() => {
// Module: crate::spec::crt_objects
// Provides: {"pre_mingw_self_contained"}
// Dependencies: {}
pub (super) fn pre_mingw_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: DynamicPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: StaticPicExe , & ["crt2.o" , "rsbegin.o"]) , (LinkOutputKind :: DynamicDylib , & ["dllcrt2.o" , "rsbegin.o"]) , (LinkOutputKind :: StaticDylib , & ["dllcrt2.o" , "rsbegin.o"]) ,]) }
};
}
