macro_rules! deps {
    () => {
        LitKind!();
    };
}

macro_rules! Lit {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Debug)] pub struct Lit { pub kind : LitKind , pub symbol : Symbol , pub suffix : Option < Symbol > , }
    };
}

Lit!();