macro_rules! deps {
    () => {
        Ctxt!();
        Container!();
        Data!();
        Derive!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a > Container < 'a > { # [doc = " Convert the raw Syn ast into a parsed container object, collecting errors in `cx`."] pub fn from_ast (cx : & Ctxt , item : & 'a syn :: DeriveInput , derive : Derive , private : & Ident ,) -> Option < Container < 'a > > { let attrs = attr :: Container :: from_ast (cx , item) ; let mut data = match & item . data { syn :: Data :: Enum (data) => { Data :: Enum (enum_from_ast (cx , & data . variants , attrs . default () , private)) } syn :: Data :: Struct (data) => { let (style , fields) = struct_from_ast (cx , & data . fields , None , attrs . default () , private) ; Data :: Struct (style , fields) } syn :: Data :: Union (_) => { cx . error_spanned_by (item , "Serde does not support derive for unions") ; return None ; } } ; match & mut data { Data :: Enum (variants) => { for variant in variants { variant . attrs . rename_by_rules (attrs . rename_all_rules ()) ; for field in & mut variant . fields { field . attrs . rename_by_rules (variant . attrs . rename_all_rules () . or (attrs . rename_all_fields_rules ()) ,) ; } } } Data :: Struct (_ , fields) => { for field in fields { field . attrs . rename_by_rules (attrs . rename_all_rules ()) ; } } } let mut item = Container { ident : item . ident . clone () , attrs , data , generics : & item . generics , original : item , } ; check :: check (cx , & mut item , derive) ; Some (item) } }
    };
}

impl_9!();