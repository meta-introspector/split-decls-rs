macro_rules! parse_cfg_attr {
    () => {
        pub fn parse_cfg_attr (cfg_attr : & Attribute , psess : & ParseSess ,) -> Option < (MetaItemInner , Vec < (AttrItem , Span) >) > { const CFG_ATTR_GRAMMAR_HELP : & str = "#[cfg_attr(condition, attribute, other_attribute, ...)]" ; const CFG_ATTR_NOTE_REF : & str = "for more information, visit \
        <https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute>" ; match cfg_attr . get_normal_item () . args { ast :: AttrArgs :: Delimited (ast :: DelimArgs { dspan , delim , ref tokens }) if ! tokens . is_empty () => { check_cfg_attr_bad_delim (psess , dspan , delim) ; match parse_in (psess , tokens . clone () , "`cfg_attr` input" , | p | p . parse_cfg_attr ()) { Ok (r) => return Some (r) , Err (e) => { e . with_help (format ! ("the valid syntax is `{CFG_ATTR_GRAMMAR_HELP}`")) . with_note (CFG_ATTR_NOTE_REF) . emit () ; } } } _ => { psess . dcx () . emit_err (errors :: MalformedCfgAttr { span : cfg_attr . span , sugg : CFG_ATTR_GRAMMAR_HELP , }) ; } } None }
    };
}

parse_cfg_attr!()