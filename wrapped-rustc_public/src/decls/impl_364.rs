macro_rules! deps {
    () => {
        AdtKind!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl Display for AdtKind { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str (match self { AdtKind :: Enum => "enum" , AdtKind :: Union => "union" , AdtKind :: Struct => "struct" , }) } }
    };
}

impl_364!();