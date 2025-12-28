macro_rules! check_attributes {
    () => {
        # [doc = " Ensures only doc comment attributes are used"] fn check_attributes (attrs : Vec < Attribute >) -> Result < Vec < Attribute > > { let inner = | attr : Attribute | { if ! attr . path () . is_ident ("doc") { Err (Error :: new (attr . span () , "attributes not supported on queries")) } else if attr . style != AttrStyle :: Outer { Err (Error :: new (attr . span () , "attributes must be outer attributes (`///`), not inner attributes" ,)) } else { Ok (attr) } } ; attrs . into_iter () . map (inner) . collect () }
    };
}

check_attributes!();