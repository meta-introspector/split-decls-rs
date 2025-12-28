macro_rules! deps {
    () => {
        SymbolPath!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl SymbolPath { fn new () -> Self { let mut result = SymbolPath { result : String :: with_capacity (64) , temp_buf : String :: with_capacity (16) } ; result . result . push_str ("_ZN") ; result } fn finalize_pending_component (& mut self) { if ! self . temp_buf . is_empty () { let _ = write ! (self . result , "{}{}" , self . temp_buf . len () , self . temp_buf) ; self . temp_buf . clear () ; } } fn finish (mut self , hash : Hash64) -> String { self . finalize_pending_component () ; let _ = write ! (self . result , "17h{hash:016x}E") ; self . result } }
    };
}

impl_19!();