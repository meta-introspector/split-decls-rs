// Generated macro for default_dcx (function)
macro_rules! Depcrate_parse_sessiondefault_dcx {
() => {
// Module: crate::parse::session
// Provides: {"default_dcx"}
// Dependencies: {}
fn default_dcx (source_map : Arc < SourceMap > , ignore_path_set : Arc < IgnorePathSet > , can_reset : Arc < AtomicBool > , show_parse_errors : bool , color : Color ,) -> DiagCtxt { let supports_color = term :: stderr () . map_or (false , | term | term . supports_color ()) ; let emit_color = if supports_color { ColorConfig :: from (color) } else { ColorConfig :: Never } ; let fallback_bundle = rustc_errors :: fallback_fluent_bundle (rustc_driver :: DEFAULT_LOCALE_RESOURCES . to_vec () , false ,) ; let emitter = Box :: new (HumanEmitter :: new (stderr_destination (emit_color) , fallback_bundle) . sm (Some (source_map . clone ())) ,) ; let emitter : Box < DynEmitter > = if ! show_parse_errors { Box :: new (SilentEmitter { fatal_emitter : emitter , fatal_note : None , emit_fatal_diagnostic : false , }) } else { emitter } ; DiagCtxt :: new (Box :: new (SilentOnIgnoredFilesEmitter { has_non_ignorable_parser_errors : false , source_map , emitter , ignore_path_set : IntoDynSyncSend (ignore_path_set) , can_reset , })) }
};
}
