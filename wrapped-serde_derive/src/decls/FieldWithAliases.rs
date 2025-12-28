macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! FieldWithAliases {
    () => {
        deps!();
        struct FieldWithAliases < 'a > { ident : Ident , aliases : & 'a BTreeSet < Name > , }
    };
}

FieldWithAliases!()