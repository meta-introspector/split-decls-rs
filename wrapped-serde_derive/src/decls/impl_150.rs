macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl Display for Symbol { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (self . 0) } }
    };
}

impl_150!()