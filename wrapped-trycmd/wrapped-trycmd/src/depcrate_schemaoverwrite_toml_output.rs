// Generated macro for overwrite_toml_output (function)
macro_rules! Depcrate_schemaoverwrite_toml_output {
() => {
// Module: crate::schema
// Provides: {"overwrite_toml_output"}
// Dependencies: {}
fn overwrite_toml_output (path : & std :: path :: Path , _id : Option < & str > , output : Option < & crate :: Data > , output_ext : & str , output_field : & str ,) -> Result < () , crate :: Error > { if let Some (output) = output { let output_path = path . with_extension (output_ext) ; if output_path . exists () { output . write_to_path (& output_path) ? ; } else if let Some (output) = output . render () { let raw = std :: fs :: read_to_string (path) . map_err (| e | format ! ("Failed to read {}: {}" , path . display () , e)) ? ; let mut doc = raw . parse :: < toml_edit :: DocumentMut > () . map_err (| e | format ! ("Failed to read {}: {}" , path . display () , e)) ? ; if let Some (output_value) = doc . get_mut (output_field) { * output_value = toml_edit :: value (output) ; } std :: fs :: write (path , doc . to_string ()) . map_err (| e | format ! ("Failed to write {}: {}" , path . display () , e)) ? ; } else { output . write_to_path (& output_path) ? ; let raw = std :: fs :: read_to_string (path) . map_err (| e | format ! ("Failed to read {}: {}" , path . display () , e)) ? ; let mut doc = raw . parse :: < toml_edit :: DocumentMut > () . map_err (| e | format ! ("Failed to read {}: {}" , path . display () , e)) ? ; doc [output_field] = toml_edit :: Item :: None ; std :: fs :: write (path , doc . to_string ()) . map_err (| e | format ! ("Failed to write {}: {}" , path . display () , e)) ? ; } } Ok (()) }
};
}
