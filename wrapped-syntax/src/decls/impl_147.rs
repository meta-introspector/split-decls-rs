macro_rules! deps {
    () => {
        PositionRepr!();
        SyntaxNode!();
        Position!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Position { pub (crate) fn parent (& self) -> SyntaxNode { self . place () . 0 } pub (crate) fn place (& self) -> (SyntaxNode , usize) { match & self . repr { PositionRepr :: FirstChild (parent) => (parent . clone () , 0) , PositionRepr :: After (child) => (child . parent () . unwrap () , child . index () + 1) , } } }
    };
}

impl_147!();