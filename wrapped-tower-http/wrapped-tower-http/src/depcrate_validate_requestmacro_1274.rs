// Generated macro for macro_1274 (macro)
macro_rules! Depcrate_validate_requestmacro_1274 {
() => {
// Module: crate::validate_request
// Provides: {"macro_1274"}
// Dependencies: {}
pin_project ! { # [project = KindProj] enum Kind < F , B > { Future { # [pin] future : F , } , Error { response : Option < Response < B >>, } , } }
};
}
