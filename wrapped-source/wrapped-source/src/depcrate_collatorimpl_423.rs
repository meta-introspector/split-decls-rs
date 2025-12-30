// Generated macro for impl_423 (impl)
macro_rules! Depcrate_collatorimpl_423 {
() => {
// Module: crate::collator
// Provides: {"impl_423"}
// Dependencies: {}
impl SourceDataProvider { fn load_toml < T > (& self , id : DataIdentifierBorrowed , suffix : & str) -> Result < & T , DataError > where for < 'de > T : serde :: Deserialize < 'de > + 'static + Send + Sync , { self . icuexport () ? . read_and_parse_toml (& format ! ("collation/{}/{}{}.toml" , self . collation_root_han () , id_to_file_name (id) , suffix)) . map_err (| e | match e . kind { DataErrorKind :: Io (std :: io :: ErrorKind :: NotFound) => { DataErrorKind :: IdentifierNotFound . into_error () } _ => e , }) } fn list_ids (& self , suffix : & str) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . icuexport () ? . list (& format ! ("collation/{}" , self . collation_root_han ())) ? . filter_map (| mut file_name | { file_name . truncate (file_name . len () - ".toml" . len ()) ; file_name . ends_with (suffix) . then (| | { file_name . truncate (file_name . len () - suffix . len ()) ; file_name }) }) . flat_map (| s | file_name_to_id (& s)) . collect ()) } }
};
}
