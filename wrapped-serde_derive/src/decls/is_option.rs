macro_rules! is_option {
    () => {
        fn is_option (ty : & syn :: Type , elem : fn (& syn :: Type) -> bool) -> bool { let path = match ungroup (ty) { syn :: Type :: Path (ty) => & ty . path , _ => { return false ; } } ; let Some (seg) = path . segments . last () else { return false ; } ; let args = match & seg . arguments { syn :: PathArguments :: AngleBracketed (bracketed) => & bracketed . args , _ => { return false ; } } ; seg . ident == "Option" && args . len () == 1 && match & args [0] { syn :: GenericArgument :: Type (arg) => elem (arg) , _ => false , } }
    };
}

is_option!()