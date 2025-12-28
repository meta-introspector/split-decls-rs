macro_rules! deps {
    () => {
        SalsaField!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 's > SalsaField < 's > { fn new (field : & 's syn :: Field) -> syn :: Result < Self > { let field_name = field . ident . as_ref () . unwrap () ; let field_name_str = field_name . to_string () ; if BANNED_FIELD_NAMES . iter () . any (| n | * n == field_name_str) { return Err (syn :: Error :: new (field_name . span () , format ! ("the field name `{field_name_str}` is disallowed in salsa structs" ,) ,)) ; } let get_name = Ident :: new (& field_name_str , field_name . span ()) ; let set_name = Ident :: new (& format ! ("set_{field_name_str}" ,) , field_name . span ()) ; let returns = Ident :: new ("clone" , field . span ()) ; let mut result = SalsaField { field , has_tracked_attr : false , returns , has_default_attr : false , has_no_eq_attr : false , maybe_update_attr : None , get_name , set_name , unknown_attrs : Default :: default () , } ; for attr in & field . attrs { let mut handled = false ; for (fa , func) in FIELD_OPTION_ATTRIBUTES { if attr . path () . is_ident (fa) { func (attr , & mut result) ? ; handled = true ; break ; } } if ! handled { result . unknown_attrs . push (attr) ; } } if ! ALLOWED_RETURN_MODES . iter () . any (| mode | mode == & result . returns . to_string ()) { return Err (syn :: Error :: new (result . returns . span () , format ! ("Invalid return mode. Allowed modes are: {ALLOWED_RETURN_MODES:?}") ,)) ; } Ok (result) } fn options (& self) -> TokenStream { let returns = & self . returns ; let backdate_ident = if self . has_no_eq_attr { syn :: Ident :: new ("no_backdate" , Span :: call_site ()) } else { syn :: Ident :: new ("backdate" , Span :: call_site ()) } ; let default_ident = if self . has_default_attr { syn :: Ident :: new ("default" , Span :: call_site ()) } else { syn :: Ident :: new ("required" , Span :: call_site ()) } ; quote ! ((# returns , # backdate_ident , # default_ident)) } }
    };
}

impl_68!();