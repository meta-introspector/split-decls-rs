macro_rules! deps {
    () => {
        LitKind!();
    };
}

macro_rules! Literal {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Literal < S > { pub symbol : Symbol , pub span : S , pub kind : LitKind , pub suffix : Option < Symbol > , }
    };
}

Literal!()