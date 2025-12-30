// Generated macro for impl_17 (impl)
macro_rules! Depcrate_attrimpl_17 {
() => {
// Module: crate::attr
// Provides: {"impl_17"}
// Dependencies: {}
impl InstrumentArgs { pub (crate) fn level (& self) -> Level { self . level . clone () . unwrap_or (Level :: Info) } pub (crate) fn target (& self) -> impl ToTokens { if let Some (ref target) = self . target { quote ! (# target) } else { quote ! (module_path ! ()) } } # [doc = " Generate \"deprecation\" warnings for any unrecognized attribute inputs"] # [doc = " that we skipped."] # [doc = ""] # [doc = " For backwards compatibility, we need to emit compiler warnings rather"] # [doc = " than errors for unrecognized inputs. Generating a fake deprecation is"] # [doc = " the only way to do this on stable Rust right now."] pub (crate) fn warnings (& self) -> impl ToTokens { let warnings = self . parse_warnings . iter () . map (| err | { let msg = format ! ("found unrecognized input, {}" , err) ; let msg = LitStr :: new (& msg , err . span ()) ; quote_spanned ! { err . span () => # [warn (deprecated)] { # [deprecated (since = "not actually deprecated" , note = # msg)] const TRACING_INSTRUMENT_WARNING : () = () ; let _ = TRACING_INSTRUMENT_WARNING ; } } }) ; quote ! { { # (# warnings) * } } } }
};
}
