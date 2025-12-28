macro_rules! deps {
    () => {
        Repr!();
        Decor!();
        KeyMut!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl KeyMut < '_ > { # [doc = " Returns the parsed key value."] pub fn get (& self) -> & str { self . key . get () } # [doc = " Returns the raw representation, if available."] pub fn as_repr (& self) -> Option < & Repr > { self . key . as_repr () } # [doc = " Returns the default raw representation."] # [cfg (feature = "display")] pub fn default_repr (& self) -> Repr { self . key . default_repr () } # [doc = " Returns a raw representation."] # [cfg (feature = "display")] pub fn display_repr (& self) -> Cow < '_ , str > { self . key . display_repr () } # [doc = " Returns the surrounding whitespace for the line entry"] pub fn leaf_decor_mut (& mut self) -> & mut Decor { self . key . leaf_decor_mut () } # [doc = " Returns the surrounding whitespace for between dots"] pub fn dotted_decor_mut (& mut self) -> & mut Decor { self . key . dotted_decor_mut () } # [doc = " Returns the surrounding whitespace for the line entry"] pub fn leaf_decor (& self) -> & Decor { self . key . leaf_decor () } # [doc = " Returns the surrounding whitespace for between dots"] pub fn dotted_decor (& self) -> & Decor { self . key . dotted_decor () } # [doc = " Auto formats the key."] pub fn fmt (& mut self) { self . key . fmt () ; } }
    };
}

impl_146!()