macro_rules! deps {
    () => {
        Parse!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T : AstNode > Parse < T > { # [doc = " Converts this parse result into a parse result for an untyped syntax tree."] pub fn to_syntax (mut self) -> Parse < SyntaxNode > { let green = self . green . take () ; let errors = self . errors . take () ; Parse { green , errors , _ty : PhantomData } } # [doc = " Gets the parsed syntax tree as a typed ast node."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the root node cannot be casted into the typed ast node"] # [doc = " (e.g. if it's an `ERROR` node)."] pub fn tree (& self) -> T { T :: cast (self . syntax_node ()) . unwrap () } # [doc = " Converts from `Parse<T>` to [`Result<T, Vec<SyntaxError>>`]."] pub fn ok (self) -> Result < T , Vec < SyntaxError > > { match self . errors () { errors if ! errors . is_empty () => Err (errors) , _ => Ok (self . tree ()) , } } }
    };
}

impl_17!()