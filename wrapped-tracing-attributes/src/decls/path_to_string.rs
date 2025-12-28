macro_rules! path_to_string {
    () => {
        fn path_to_string (path : & Path) -> String { use std :: fmt :: Write ; let mut res = String :: with_capacity (path . segments . len () * 5) ; for i in 0 .. path . segments . len () { write ! (& mut res , "{}" , path . segments [i] . ident) . expect ("writing to a String should never fail") ; if i < path . segments . len () - 1 { res . push_str ("::") ; } } res }
    };
}

path_to_string!()