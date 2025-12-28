macro_rules! deps {
    () => {
        Decode!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < F , R , E > core :: fmt :: Display for Decode < F > where F : Clone + FnOnce () -> R , R : IntoIterator < Item = core :: result :: Result < char , E > > , { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use core :: fmt :: Write ; let iter = self . 0 . clone () ; for c in iter () . into_iter () { f . write_char (c . unwrap_or (core :: char :: REPLACEMENT_CHARACTER)) ? } Ok (()) } }
    };
}

impl_94!()