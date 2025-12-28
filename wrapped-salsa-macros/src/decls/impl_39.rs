macro_rules! deps {
    () => {
        Macro!();
        SalsaStruct!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Macro { # [allow (non_snake_case)] fn try_macro (& self) -> syn :: Result < TokenStream > { let salsa_struct = SalsaStruct :: new (& self . struct_item , & self . args) ? ; let attrs = & self . struct_item . attrs ; let vis = & self . struct_item . vis ; let struct_ident = & self . struct_item . ident ; let new_fn = salsa_struct . constructor_name () ; let field_ids = salsa_struct . field_ids () ; let field_indices = salsa_struct . field_indices () ; let num_fields = salsa_struct . num_fields () ; let field_vis = salsa_struct . field_vis () ; let field_getter_ids = salsa_struct . field_getter_ids () ; let field_setter_ids = salsa_struct . field_setter_ids () ; let required_fields = salsa_struct . required_fields () ; let field_options = salsa_struct . field_options () ; let field_tys = salsa_struct . field_tys () ; let field_durability_ids = salsa_struct . field_durability_ids () ; let field_attrs = salsa_struct . field_attrs () ; let is_singleton = self . args . singleton . is_some () ; let generate_debug_impl = salsa_struct . generate_debug_impl () ; let heap_size_fn = self . args . heap_size_fn . iter () ; let persist = self . args . persist () ; let serialize_fn = salsa_struct . serialize_fn () ; let deserialize_fn = salsa_struct . deserialize_fn () ; let zalsa = self . hygiene . ident ("zalsa") ; let zalsa_struct = self . hygiene . ident ("zalsa_struct") ; let Configuration = self . hygiene . ident ("Configuration") ; let Builder = self . hygiene . ident ("Builder") ; let CACHE = self . hygiene . ident ("CACHE") ; let Db = self . hygiene . ident ("Db") ; Ok (crate :: debug :: dump_tokens (struct_ident , quote ! { salsa :: plumbing :: setup_input_struct ! (attrs : [# (# attrs) ,*] , vis : # vis , Struct : # struct_ident , new_fn : # new_fn , field_options : [# (# field_options) ,*] , field_ids : [# (# field_ids) ,*] , field_getters : [# (# field_vis # field_getter_ids) ,*] , field_setters : [# (# field_vis # field_setter_ids) ,*] , field_tys : [# (# field_tys) ,*] , field_indices : [# (# field_indices) ,*] , field_attrs : [# ([# (# field_attrs) ,*]) ,*] , required_fields : [# (# required_fields) ,*] , field_durability_ids : [# (# field_durability_ids) ,*] , num_fields : # num_fields , is_singleton : # is_singleton , generate_debug_impl : # generate_debug_impl , heap_size_fn : # (# heap_size_fn) *, persist : # persist , serialize_fn : # (# serialize_fn) *, deserialize_fn : # (# deserialize_fn) *, unused_names : [# zalsa , # zalsa_struct , # Configuration , # Builder , # CACHE , # Db ,]) ; } ,)) } }
    };
}

impl_39!()