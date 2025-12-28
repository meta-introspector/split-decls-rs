macro_rules! deps {
    () => {
        Array!();
        VisitMut!();
    };
}

macro_rules! visit_array_mut {
    () => {
        deps!();
        pub fn visit_array_mut < V > (v : & mut V , node : & mut Array) where V : VisitMut + ? Sized , { for value in node . iter_mut () { v . visit_value_mut (value) ; } }
    };
}

visit_array_mut!();