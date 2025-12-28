macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Error { # [cfg (feature = "parse")] pub (crate) fn new (input : alloc :: sync :: Arc < str > , error : toml_parser :: ParseError) -> Self { let mut message = String :: new () ; message . push_str (error . description ()) ; if let Some (expected) = error . expected () { message . push_str (", expected ") ; if expected . is_empty () { message . push_str ("nothing") ; } else { for (i , expected) in expected . iter () . enumerate () { if i != 0 { message . push_str (", ") ; } match expected { toml_parser :: Expected :: Literal (desc) => { message . push_str (& render_literal (desc)) ; } toml_parser :: Expected :: Description (desc) => message . push_str (desc) , _ => message . push_str ("etc") , } } } } let span = error . unexpected () . map (| span | span . start () .. span . end ()) ; Self { message , input : Some (input) , keys : Vec :: new () , span , } } pub (crate) fn custom < T > (msg : T , span : Option < core :: ops :: Range < usize > >) -> Self where T : core :: fmt :: Display , { Self { message : msg . to_string () , input : None , keys : Vec :: new () , span , } } pub (crate) fn add_key (& mut self , key : String) { self . keys . insert (0 , key) ; } # [doc = " What went wrong"] pub fn message (& self) -> & str { & self . message } # [doc = " The start/end index into the original document where the error occurred"] pub fn span (& self) -> Option < core :: ops :: Range < usize > > { self . span . clone () } pub (crate) fn set_span (& mut self , span : Option < core :: ops :: Range < usize > >) { self . span = span ; } # [doc = " Provide the encoded TOML the error applies to"] pub fn set_input (& mut self , input : Option < & str >) { self . input = input . map (| s | s . into ()) ; } }
    };
}

impl_163!()