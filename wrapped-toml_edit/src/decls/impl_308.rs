macro_rules! deps {
    () => {
        TableMapAccess!();
        TableDeserializer!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl TableMapAccess { pub (crate) fn new (input : TableDeserializer) -> Self { Self { iter : input . items . into_iter () , span : input . span , value : None , } } }
    };
}

impl_308!();