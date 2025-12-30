// Generated macro for impl_431 (impl)
macro_rules! Depcrate_configimpl_431 {
() => {
// Module: crate::config
// Provides: {"impl_431"}
// Dependencies: {}
impl PartialConfig { pub fn to_toml (& self) -> Result < String , ToTomlError > { let mut cloned = self . clone () ; cloned . file_lines = None ; cloned . verbose = None ; cloned . width_heuristics = None ; cloned . print_misformatted_file_names = None ; cloned . merge_imports = None ; cloned . fn_args_layout = None ; cloned . hide_parse_errors = None ; :: toml :: to_string (& cloned) . map_err (ToTomlError) } pub (super) fn to_parsed_config (self , style_edition_override : Option < StyleEdition > , edition_override : Option < Edition > , version_override : Option < Version > , dir : & Path ,) -> Config { Config :: default_for_possible_style_edition (style_edition_override . or (self . style_edition) , edition_override . or (self . edition) , version_override . or (self . version) ,) . fill_from_parsed_config (self , dir) } }
};
}
