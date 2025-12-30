// Generated macro for check_name_constraints (function)
macro_rules! Depcrate_subject_namecheck_name_constraints {
() => {
// Module: crate::subject_name
// Provides: {"check_name_constraints"}
// Dependencies: {}
pub (crate) fn check_name_constraints (constraints : Option < & mut untrusted :: Reader < '_ > > , path : & PathNode < '_ > , budget : & mut Budget ,) -> Result < () , Error > { let constraints = match constraints { Some (input) => input , None => return Ok (()) , } ; fn parse_subtrees < 'b > (inner : & mut untrusted :: Reader < 'b > , subtrees_tag : der :: Tag ,) -> Result < Option < untrusted :: Input < 'b > > , Error > { if ! inner . peek (subtrees_tag . into ()) { return Ok (None) ; } der :: expect_tag (inner , subtrees_tag) . map (Some) } let permitted_subtrees = parse_subtrees (constraints , der :: Tag :: ContextSpecificConstructed0) ? ; let excluded_subtrees = parse_subtrees (constraints , der :: Tag :: ContextSpecificConstructed1) ? ; for path in path . iter () { let result = NameIterator :: new (path . cert . subject_alt_name) . find_map (| result | { let name = match result { Ok (name) => name , Err (err) => return Some (Err (err)) , } ; check_presented_id_conforms_to_constraints (name , permitted_subtrees , excluded_subtrees , budget ,) }) ; if let Some (Err (err)) = result { return Err (err) ; } let result = check_presented_id_conforms_to_constraints (GeneralName :: DirectoryName , permitted_subtrees , excluded_subtrees , budget ,) ; if let Some (Err (err)) = result { return Err (err) ; } } Ok (()) }
};
}
