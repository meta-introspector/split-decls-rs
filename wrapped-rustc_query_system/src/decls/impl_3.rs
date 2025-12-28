macro_rules! deps {
    () => {
        Value!();
        Cache!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < Key , Value > Cache < Key , Value > { # [doc = " Actually frees the underlying memory in contrast to what stdlib containers do on `clear`"] pub fn clear (& self) { * self . hashmap . borrow_mut () = Default :: default () ; } }
    };
}

impl_3!()