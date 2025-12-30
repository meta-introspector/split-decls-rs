// Generated macro for tests (module)
macro_rules! Depcrate_service_exttests {
() => {
// Module: crate::service_ext
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "fs" , feature = "add-extension"))] mod tests { use super :: ServiceExt ; use crate :: services ; # [allow (dead_code)] fn test_type_inference () { let _svc = services :: fs :: ServeDir :: new (".") . add_extension ("&'static str") ; } }
};
}
