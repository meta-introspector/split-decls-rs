macro_rules! deps {
    () => {
        DeString!();
        DeValue!();
        IntoIter!();
    };
}

macro_rules! TableMapAccess {
    () => {
        deps!();
        pub (crate) struct TableMapAccess < 'i > { iter : IntoIter < Spanned < DeString < 'i > > , Spanned < DeValue < 'i > > > , span : core :: ops :: Range < usize > , value : Option < (Spanned < DeString < 'i > > , Spanned < DeValue < 'i > >) > , }
    };
}

TableMapAccess!();