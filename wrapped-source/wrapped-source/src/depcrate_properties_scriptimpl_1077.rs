// Generated macro for impl_1077 (impl)
macro_rules! Depcrate_properties_scriptimpl_1077 {
() => {
// Module: crate::properties::script
// Provides: {"impl_1077"}
// Dependencies: {}
impl DataProvider < PropertyScriptWithExtensionsV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < PropertyScriptWithExtensionsV1 > , DataError > { self . check_req :: < PropertyScriptWithExtensionsV1 > (req) ? ; let scx_data = self . icuexport () ? . read_and_parse_toml :: < super :: uprops_serde :: script_extensions :: Main > (& format ! ("uprops/{}/scx.toml" , self . trie_type () ,)) ? . script_extensions . first () . ok_or_else (| | DataError :: custom ("Could not parse Script_Extensions data from TOML")) ? ; let cpt_data = & scx_data . code_point_trie ; let scx_array_data = & scx_data . script_code_array ; let trie = CodePointTrie :: < ScriptWithExt > :: try_from (cpt_data) . map_err (| e | { DataError :: custom ("Could not parse CodePointTrie TOML") . with_display_context (& e) }) ? ; let ule_scx_array_data : Vec < ZeroVec < Script > > = scx_array_data . iter () . map (| v | { v . iter () . copied () . map (Script :: from_icu4c_value) . collect :: < ZeroVec < Script > > () }) . collect :: < Vec < ZeroVec < Script > > > () ; let scx_vzv : VarZeroVec < ZeroSlice < Script > > = VarZeroVec :: from (ule_scx_array_data . as_slice ()) ; let data_struct = ScriptWithExtensionsProperty { trie , extensions : scx_vzv , } ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data_struct) , }) } }
};
}
