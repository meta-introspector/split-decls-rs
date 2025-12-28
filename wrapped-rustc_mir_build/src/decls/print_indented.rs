macro_rules! deps {
    () => {
        ThirPrinter!();
    };
}

macro_rules! print_indented {
    () => {
        deps!();
        macro_rules ! print_indented { ($ writer : ident , $ s : expr , $ indent_lvl : expr) => { $ writer . indent ($ indent_lvl) ; writeln ! ($ writer , "{}" , $ s) . expect ("unable to write to ThirPrinter") ; } ; }
    };
}

print_indented!();