macro_rules! deps {
    () => {
        Element!();
        PositionRepr!();
        Position!();
        SyntaxNode!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Position { pub fn after (elem : impl Element) -> Position { let repr = PositionRepr :: After (elem . syntax_element ()) ; Position { repr } } pub fn before (elem : impl Element) -> Position { let elem = elem . syntax_element () ; let repr = match elem . prev_sibling_or_token () { Some (it) => PositionRepr :: After (it) , None => PositionRepr :: FirstChild (elem . parent () . unwrap ()) , } ; Position { repr } } pub fn first_child_of (node : & (impl Into < SyntaxNode > + Clone)) -> Position { let repr = PositionRepr :: FirstChild (node . clone () . into ()) ; Position { repr } } pub fn last_child_of (node : & (impl Into < SyntaxNode > + Clone)) -> Position { let node = node . clone () . into () ; let repr = match node . last_child_or_token () { Some (it) => PositionRepr :: After (it) , None => PositionRepr :: FirstChild (node) , } ; Position { repr } } }
    };
}

impl_149!();