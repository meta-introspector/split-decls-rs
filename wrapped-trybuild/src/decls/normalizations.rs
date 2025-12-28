macro_rules! deps {
    () => {
        Variations!();
    };
}

macro_rules! normalizations {
    () => {
        deps!();
        macro_rules ! normalizations { ($ ($ name : ident ,) *) => { # [derive (PartialOrd , PartialEq , Copy , Clone)] enum Normalization { $ ($ name ,) * } impl Normalization { const ALL : &'static [Self] = & [$ ($ name) ,*] ; } impl Default for Variations { fn default () -> Self { Variations { variations : [$ (($ name , String :: new ()) . 1) ,*] , } } } } ; }
    };
}

normalizations!();