macro_rules! deps {
    () => {
        Parser!();
        ParseMode!();
        Piece!();
    };
}

macro_rules! asm_linespans {
    () => {
        deps!();
        # [test] fn asm_linespans () { let asm_pre = r###"r"
        .intel_syntax noprefix
        nop""### ; let asm = r"
        .intel_syntax noprefix
        nop" ; let mut parser = Parser :: new (asm , Some (0) , Some (asm_pre . into ()) , false , ParseMode :: InlineAsm) ; assert ! (parser . is_source_literal) ; assert_eq ! (parser . by_ref () . collect ::< Vec < Piece <'static >>> () , & [Lit ("\n        .intel_syntax noprefix\n        nop")]) ; assert_eq ! (parser . line_spans , & [2 .. 2 , 11 .. 33 , 42 .. 45]) ; }
    };
}

asm_linespans!()