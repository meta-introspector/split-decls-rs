macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! ForeignItemKind {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ForeignItemKind { Fn (FnDef) , Static (StaticDef) , Type (Ty) , }
    };
}

ForeignItemKind!();