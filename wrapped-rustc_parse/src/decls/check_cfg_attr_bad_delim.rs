macro_rules! check_cfg_attr_bad_delim {
    () => {
        fn check_cfg_attr_bad_delim (psess : & ParseSess , span : DelimSpan , delim : Delimiter) { if let Delimiter :: Parenthesis = delim { return ; } psess . dcx () . emit_err (errors :: CfgAttrBadDelim { span : span . entire () , sugg : errors :: MetaBadDelimSugg { open : span . open , close : span . close } , }) ; }
    };
}

check_cfg_attr_bad_delim!()