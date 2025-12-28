macro_rules! panic_in_drop {
    () => {
        macro_rules ! panic_in_drop { ($ ($ arg : tt) *) => { if ! std :: thread :: panicking () { panic ! ($ ($ arg) *) } else { let thread = std :: thread :: current () ; eprintln ! ("thread '{thread}' attempted to panic at '{msg}', {file}:{line}:{col}\n\
                note: we were already unwinding due to a previous panic." , thread = thread . name () . unwrap_or ("<unnamed>") , msg = format_args ! ($ ($ arg) *) , file = file ! () , line = line ! () , col = column ! () ,) ; } } }
    };
}

panic_in_drop!();