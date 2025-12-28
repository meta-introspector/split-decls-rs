macro_rules! deps {
    () => {
        DeclsVisitor!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'a > DeclsVisitor < 'a > { pub fn new (source_file : PathBuf , crate_name : String , verbosity : u8 , file_extern_crates : HashSet < String > , warnings : & 'a mut Vec < String > ,) -> Self { DeclsVisitor { declarations : HashMap :: new () , fn_count : 0 , struct_count : 0 , enum_count : 0 , const_count : 0 , static_count : 0 , macro_count : 0 , mod_count : 0 , trait_count : 0 , trait_alias_count : 0 , type_count : 0 , union_count : 0 , other_item_count : 0 , source_file , crate_name , verbosity , file_extern_crates , warnings , } } fn is_proc_macro_item (attrs : & [syn :: Attribute]) -> bool { attrs . iter () . any (| attr | { attr . path () . is_ident ("proc_macro") || attr . path () . is_ident ("proc_macro_derive") || attr . path () . is_ident ("proc_macro_attribute") }) } fn extract_identifiers_from_type (& self , ty : & syn :: Type) -> HashSet < String > { let mut identifiers = HashSet :: new () ; match ty { syn :: Type :: Path (type_path) => { for segment in type_path . path . segments . iter () { identifiers . insert (segment . ident . to_string ()) ; } } _ => { } } identifiers } fn extract_identifiers_from_expr (& self , expr : & syn :: Expr) -> HashSet < String > { let mut identifiers = HashSet :: new () ; match expr { syn :: Expr :: Path (expr_path) => { for segment in expr_path . path . segments . iter () { identifiers . insert (segment . ident . to_string ()) ; } } syn :: Expr :: Call (expr_call) => { identifiers . extend (self . extract_identifiers_from_expr (& expr_call . func)) ; } syn :: Expr :: MethodCall (expr_method_call) => { identifiers . insert (expr_method_call . method . to_string ()) ; identifiers . extend (self . extract_identifiers_from_expr (& expr_method_call . receiver)) ; } _ => { } } identifiers } }
    };
}

impl_19!();