macro_rules! deps {
    () => {
        VariantInfo!();
    };
}

macro_rules! get_ty_params {
    () => {
        deps!();
        # [doc = " Helper function used by the `VariantInfo` constructor. Walks all of the types"] # [doc = " in `field` and returns a list of the type parameters from `ty_params` which"] # [doc = " are referenced in the field."] fn get_ty_params (field : & Field , generics : & Generics) -> Vec < bool > { struct BoundTypeLocator < 'a > { result : Vec < bool > , generics : & 'a Generics , } impl < 'a > Visit < 'a > for BoundTypeLocator < 'a > { fn visit_ident (& mut self , id : & Ident) { for (idx , i) in self . generics . params . iter () . enumerate () { if let GenericParam :: Type (tparam) = i { if tparam . ident == * id { self . result [idx] = true ; } } } } fn visit_type_macro (& mut self , x : & 'a TypeMacro) { for r in & mut self . result { * r = true ; } visit :: visit_type_macro (self , x) ; } } let mut btl = BoundTypeLocator { result : vec ! [false ; generics . params . len ()] , generics , } ; btl . visit_type (& field . ty) ; btl . result }
    };
}

get_ty_params!();