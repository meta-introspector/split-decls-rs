macro_rules! deps {
    () => {
        Cache!();
        Value!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < Key : Clone , Value : Clone > Clone for Cache < Key , Value > { fn clone (& self) -> Self { Self { hashmap : Lock :: new (self . hashmap . borrow () . clone ()) } } }
    };
}

impl_1!();