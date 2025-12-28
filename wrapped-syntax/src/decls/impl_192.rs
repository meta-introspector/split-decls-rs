macro_rules! deps {
    () => {
        SyntaxNode!();
        Parse!();
        SyntaxError!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < T > Parse < T > { fn new (green : GreenNode , errors : Vec < SyntaxError >) -> Parse < T > { Parse { green : Some (green) , errors : if errors . is_empty () { None } else { Some (errors . into ()) } , _ty : PhantomData , } } pub fn syntax_node (& self) -> SyntaxNode { SyntaxNode :: new_root (self . green . as_ref () . unwrap () . clone ()) } pub fn errors (& self) -> Vec < SyntaxError > { let mut errors = if let Some (e) = self . errors . as_deref () { e . to_vec () } else { vec ! [] } ; validation :: validate (& self . syntax_node () , & mut errors) ; errors } }
    };
}

impl_192!();