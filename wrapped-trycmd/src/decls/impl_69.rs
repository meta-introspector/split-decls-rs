macro_rules! deps {
    () => {
        Stdio!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Stdio { fn as_str (& self) -> & str { match self { Self :: Stdout => "stdout" , Self :: Stderr => "stderr" , } } }
    };
}

impl_69!()