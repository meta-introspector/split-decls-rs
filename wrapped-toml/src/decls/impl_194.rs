macro_rules! deps {
    () => {
        DeArray!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl core :: fmt :: Debug for DeArray < '_ > { # [inline] fn fmt (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . items . fmt (formatter) } }
    };
}

impl_194!()