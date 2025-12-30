// Generated macro for Step (struct)
macro_rules! Depcrate_schemaStep {
() => {
// Module: crate::schema
// Provides: {"Step"}
// Dependencies: {}
# [derive (Clone , Default , Debug , PartialEq , Eq)] pub (crate) struct Step { pub (crate) id : Option < String > , pub (crate) bin : Option < Bin > , pub (crate) args : Vec < String > , pub (crate) env : Env , pub (crate) stdin : Option < crate :: Data > , pub (crate) stderr_to_stdout : bool , pub (crate) expected_status_source : Option < usize > , pub (crate) expected_status : Option < CommandStatus > , pub (crate) expected_stdout_source : Option < std :: ops :: Range < usize > > , pub (crate) expected_stdout : Option < crate :: Data > , pub (crate) expected_stderr_source : Option < std :: ops :: Range < usize > > , pub (crate) expected_stderr : Option < crate :: Data > , pub (crate) binary : bool , pub (crate) timeout : Option < std :: time :: Duration > , }
};
}
