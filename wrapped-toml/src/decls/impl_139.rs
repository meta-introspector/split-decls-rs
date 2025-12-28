macro_rules! deps {
    () => {
        TableDeserializer!();
        TableMapAccess!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'i > TableMapAccess < 'i > { pub (crate) fn new (input : TableDeserializer < 'i >) -> Self { Self { iter : input . items . into_iter () , span : input . span , value : None , } } }
    };
}

impl_139!();