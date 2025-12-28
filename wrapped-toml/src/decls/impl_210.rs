macro_rules! deps {
    () => {
        TomlSink!();
        DeTable!();
        Error!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < 'i > DeTable < 'i > { # [doc = " Parse a TOML document"] pub fn parse (input : & 'i str) -> Result < Spanned < Self > , crate :: de :: Error > { let source = toml_parser :: Source :: new (input) ; let mut errors = crate :: de :: error :: TomlSink :: < Option < _ > > :: new (source) ; let value = crate :: de :: parser :: parse_document (source , & mut errors) ; if let Some (err) = errors . into_inner () { Err (err) } else { Ok (value) } } # [doc = " Parse a TOML document, with best effort recovery on error"] pub fn parse_recoverable (input : & 'i str) -> (Spanned < Self > , Vec < crate :: de :: Error >) { let source = toml_parser :: Source :: new (input) ; let mut errors = crate :: de :: error :: TomlSink :: < Vec < _ > > :: new (source) ; let value = crate :: de :: parser :: parse_document (source , & mut errors) ; (value , errors . into_inner ()) } # [doc = " Ensure no data is borrowed"] pub fn make_owned (& mut self) { self . mut_entries (| k , v | { let owned = core :: mem :: take (k . get_mut ()) ; * k . get_mut () = Cow :: Owned (owned . into_owned ()) ; v . get_mut () . make_owned () ; }) ; } }
    };
}

impl_210!()