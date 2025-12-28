macro_rules! deps {
    () => {
        Visit!();
        Array!();
    };
}

macro_rules! visit_array {
    () => {
        deps!();
        pub fn visit_array < 'doc , V > (v : & mut V , node : & 'doc Array) where V : Visit < 'doc > + ? Sized , { for value in node . iter () { v . visit_value (value) ; } }
    };
}

visit_array!()