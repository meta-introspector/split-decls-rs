// Generated macro for extract_result_info (function)
macro_rules! Depcrate_verification_androidextract_result_info {
() => {
// Module: crate::verification::android
// Provides: {"extract_result_info"}
// Dependencies: {}
fn extract_result_info (env : & mut JNIEnv < '_ > , result : JObject < '_ > ,) -> (VerifierStatus , Option < String >) { let status_code = env . get_field (& result , "code" , "I") . and_then (| code | code . i ()) . unwrap () ; let status = match status_code { 0 => VerifierStatus :: Ok , 1 => VerifierStatus :: Unavailable , 2 => VerifierStatus :: Expired , 3 => VerifierStatus :: UnknownCert , 4 => VerifierStatus :: Revoked , 5 => VerifierStatus :: InvalidEncoding , 6 => VerifierStatus :: InvalidExtension , i => unreachable ! ("unknown status code: {i}") , } ; let msg = env . get_field (result , "message" , "Ljava/lang/String;") . and_then (| m | m . l ()) . map (| s | { if s . is_null () { None } else { JavaStr :: from_env (env , & s . into ()) . ok () . map (String :: from) } }) . unwrap () ; (status , msg) }
};
}
