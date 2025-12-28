macro_rules! ast_enum_of_structs_impl {
    () => {
        macro_rules ! ast_enum_of_structs_impl { ($ name : ident { $ ($ (# [cfg $ cfg_attr : tt]) * $ (# [doc $ ($ doc_attr : tt) *]) * $ variant : ident $ (($ member : ident)) *,) * }) => { $ ($ (ast_enum_from_struct ! ($ name ::$ variant , $ member) ;) *) * } ; }
    };
}

ast_enum_of_structs_impl!()