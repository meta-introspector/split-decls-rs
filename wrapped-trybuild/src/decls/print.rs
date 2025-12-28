macro_rules! print {
    () => {
        # [deny (unused_macros)] macro_rules ! print { ($ ($ args : tt) *) => { { use std :: io :: Write ; let _ = std :: write ! ($ crate :: term :: lock () , $ ($ args) *) ; } } ; }
    };
}

print!()