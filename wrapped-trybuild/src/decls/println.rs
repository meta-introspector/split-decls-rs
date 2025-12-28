macro_rules! println {
    () => {
        # [deny (unused_macros)] macro_rules ! println { ($ ($ args : tt) *) => { { use std :: io :: Write ; let _ = std :: writeln ! ($ crate :: term :: lock () , $ ($ args) *) ; } } ; }
    };
}

println!()