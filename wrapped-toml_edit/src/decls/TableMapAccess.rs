macro_rules! deps {
    () => {
        Item!();
        IntoIter!();
        Key!();
    };
}

macro_rules! TableMapAccess {
    () => {
        deps!();
        pub (crate) struct TableMapAccess { iter : indexmap :: map :: IntoIter < crate :: Key , crate :: Item > , span : Option < std :: ops :: Range < usize > > , value : Option < (crate :: Key , crate :: Item) > , }
    };
}

TableMapAccess!();