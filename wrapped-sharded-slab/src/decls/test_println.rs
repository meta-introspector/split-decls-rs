macro_rules! deps {
    () => {
        DefaultConfig!();
        Tid!();
    };
}

macro_rules! test_println {
    () => {
        deps!();
        macro_rules ! test_println { ($ ($ arg : tt) *) => { if cfg ! (test) && cfg ! (slab_print) { if std :: thread :: panicking () { println ! ("[PANIC {:>17}:{:<3}] {}" , file ! () , line ! () , format_args ! ($ ($ arg) *)) } else { println ! ("[{:?} {:>17}:{:<3}] {}" , crate :: Tid ::< crate :: DefaultConfig >:: current () , file ! () , line ! () , format_args ! ($ ($ arg) *)) } } } }
    };
}

test_println!()