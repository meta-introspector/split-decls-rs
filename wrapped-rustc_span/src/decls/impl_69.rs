macro_rules! deps {
    () => {
        AstPass!();
        ExpnKind!();
        MacroKind!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl ExpnKind { pub fn descr (& self) -> String { match * self { ExpnKind :: Root => kw :: PathRoot . to_string () , ExpnKind :: Macro (macro_kind , name) => match macro_kind { MacroKind :: Bang => format ! ("{name}!") , MacroKind :: Attr => format ! ("#[{name}]") , MacroKind :: Derive => format ! ("#[derive({name})]") , } , ExpnKind :: AstPass (kind) => kind . descr () . to_string () , ExpnKind :: Desugaring (kind) => format ! ("desugaring of {}" , kind . descr ()) , } } }
    };
}

impl_69!();