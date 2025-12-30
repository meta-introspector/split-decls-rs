// Generated macro for impl_46 (impl)
macro_rules! Depcrate_struct_metaimpl_46 {
() => {
// Module: crate::struct_meta
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a > Param < 'a > { fn from_field (index : usize , field : & 'a Field) -> Result < Self > { let mut name = None ; let mut name_specified = false ; let mut unnamed = false ; for attr in & field . attrs { if attr . path () . is_ident ("struct_meta") { let a = attr . parse_args :: < ArgsForField > () ? ; if let Some (a_name) = a . name { name = Some ((a_name . value () , a_name . span ())) ; name_specified = true ; } if a . unnamed { unnamed = true ; } } } if name . is_none () { if let Some (ident) = & field . ident { name = Some ((ident . unraw () . to_string () , ident . span ())) ; } } if unnamed { name = None ; } let mut is_map = false ; let mut is_option = false ; let ty = if let (false , Some (ty)) = (name_specified , get_hash_map_string_element (& field . ty)) { is_map = true ; ty } else if let Some (ty) = get_option_element (& field . ty) { is_option = true ; ty } else { & field . ty } ; let info = ParamInfo :: new (index , field , ty) ; let ty = NamedParamType :: from_type (ty , ! is_map && ! is_option) ; let this = if is_map { Param :: Rest (RestParam { info , ty }) } else if let Some ((name , name_span)) = name { Param :: Named (NamedParam { info , name , name_span , ty , is_option , }) } else if let NamedParamType :: Value { ty , is_vec } = ty { Param :: Unnamed (UnnamedParam { info , ty , is_option , is_vec , }) } else { bail ! (info . span () , "this field type cannot be used as unnamed parameter.") } ; Ok (this) } }
};
}
