macro_rules! deps {
    () => {
        RenameAllRules!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl RenameAllRules { # [doc = " Returns a new `RenameAllRules` with the individual rules of `self` and"] # [doc = " `other_rules` joined by `RenameRules::or`."] pub fn or (self , other_rules : Self) -> Self { Self { serialize : self . serialize . or (other_rules . serialize) , deserialize : self . deserialize . or (other_rules . deserialize) , } } }
    };
}

impl_23!()