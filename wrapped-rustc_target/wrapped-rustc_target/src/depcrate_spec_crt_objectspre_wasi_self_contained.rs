// Generated macro for pre_wasi_self_contained (function)
macro_rules! Depcrate_spec_crt_objectspre_wasi_self_contained {
() => {
// Module: crate::spec::crt_objects
// Provides: {"pre_wasi_self_contained"}
// Dependencies: {}
pub (super) fn pre_wasi_self_contained () -> CrtObjects { new (& [(LinkOutputKind :: DynamicNoPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: DynamicPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: StaticNoPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: StaticPicExe , & ["crt1-command.o"]) , (LinkOutputKind :: WasiReactorExe , & ["crt1-reactor.o"]) ,]) }
};
}
