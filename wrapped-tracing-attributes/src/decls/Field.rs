macro_rules! deps {
    () => {
        FieldKind!();
    };
}

macro_rules! Field {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct Field { pub (crate) name : Punctuated < Ident , Token ! [.] > , pub (crate) value : Option < Expr > , pub (crate) kind : FieldKind , }
    };
}

Field!();