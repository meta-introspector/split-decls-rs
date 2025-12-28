macro_rules! deps {
    () => {
        HierarchicalLayer!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl HierarchicalLayer < fn () -> io :: Stderr > { pub fn new (indent_amount : usize) -> Self { let ansi = io :: stderr () . is_terminal () ; let config = Config { ansi , indent_amount , .. Default :: default () } ; Self { make_writer : io :: stderr , bufs : Mutex :: new (Buffers :: new ()) , config , timer : () , } } }
    };
}

impl_7!()