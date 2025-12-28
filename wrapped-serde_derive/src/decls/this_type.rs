macro_rules! deps {
    () => {
        Container!();
    };
}

macro_rules! this_type {
    () => {
        deps!();
        pub fn this_type (cont : & Container) -> Path { if let Some (remote) = cont . attrs . remote () { let mut this = remote . clone () ; for segment in & mut this . segments { if let PathArguments :: AngleBracketed (arguments) = & mut segment . arguments { arguments . colon2_token = None ; } } this } else { Path :: from (cont . ident . clone ()) } }
    };
}

this_type!();