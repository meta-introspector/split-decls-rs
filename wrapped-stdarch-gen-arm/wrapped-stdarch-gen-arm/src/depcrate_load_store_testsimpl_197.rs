// Generated macro for impl_197 (impl)
macro_rules! Depcrate_load_store_testsimpl_197 {
() => {
// Module: crate::load_store_tests
// Provides: {"impl_197"}
// Dependencies: {}
impl LdIntrCharacteristics { fn new (intr : & Intrinsic) -> Result < LdIntrCharacteristics , String > { let input = intr . input . types . first () . unwrap () . get (0) . unwrap () ; let load_type = input . get (intr . test . get_typeset_index () . unwrap ()) . and_then (InputType :: typekind) . and_then (TypeKind :: base_type) . unwrap () ; let ret_type = intr . signature . return_type . clone () ; let name = intr . signature . fn_name () . to_string () ; let tuple_len = name . chars () . find (| c | c . is_numeric ()) . and_then (| c | c . to_digit (10)) . unwrap_or (1) as usize ; let uses_ffr = name . starts_with ("svldff") || name . starts_with ("svldnf") ; let is_prf = name . starts_with ("svprf") ; let replicate_width = if name . starts_with ("svld1ro") { Some (256) } else if name . starts_with ("svld1rq") { Some (128) } else { None } ; let get_ty_of_arg = | name : & str | { intr . signature . arguments . iter () . find (| a | a . name . to_string () == name) . map (| a | a . kind . clone ()) } ; let gather_bases_type = get_ty_of_arg ("bases") ; let gather_offset_type = get_ty_of_arg ("offset") ; let gather_index_type = get_ty_of_arg ("index") ; let gather_offsets_type = get_ty_of_arg ("offsets") ; let gather_indices_type = get_ty_of_arg ("indices") ; Ok (LdIntrCharacteristics { load_type : * load_type , ret_type , tuple_len , vnum : name . contains ("vnum") , uses_ffr , is_prf , replicate_width , gather_bases_type , gather_offset_type , gather_index_type , gather_offsets_type , gather_indices_type , }) } }
};
}
