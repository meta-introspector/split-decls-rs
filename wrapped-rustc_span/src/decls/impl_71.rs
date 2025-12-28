macro_rules! deps {
    () => {
        MacroKind!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl MacroKind { pub fn descr (self) -> & 'static str { match self { MacroKind :: Bang => "macro" , MacroKind :: Attr => "attribute macro" , MacroKind :: Derive => "derive macro" , } } pub fn descr_expected (self) -> & 'static str { match self { MacroKind :: Attr => "attribute" , _ => self . descr () , } } pub fn article (self) -> & 'static str { match self { MacroKind :: Attr => "an" , _ => "a" , } } }
    };
}

impl_71!()