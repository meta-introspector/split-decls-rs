macro_rules! deps {
    () => {
        Argument!();
        Piece!();
        ParseMode!();
        Parser!();
    };
}

macro_rules! format_raw {
    () => {
        deps!();
        # [test] fn format_raw () { let snippet = r###"r#"assertion `left {op} right` failed"#"### . into () ; let source = r#"assertion `left {op} right` failed"# ; let parser = Parser :: new (source , Some (1) , Some (snippet) , true , ParseMode :: Format) ; let expected = & [Lit ("assertion `left ") , NextArgument (Box :: new (Argument { position : ArgumentNamed ("op") , position_span : 20 .. 22 , format : fmtdflt () , })) , Lit (" right` failed") ,] ; assert_eq ! (parser . collect ::< Vec < Piece <'static >>> () , expected) ; }
    };
}

format_raw!()