macro_rules! deps {
    () => {
        SynToken!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < S > SynToken < S > { fn token (& self) -> & SyntaxToken { match self { SynToken :: Ordinary (it) | SynToken :: Punct { token : it , offset : _ } => it , SynToken :: Leaf (_) => unreachable ! () , } } }
    };
}

impl_28!()