macro_rules! ast_enum_from_struct {
    () => {
        macro_rules ! ast_enum_from_struct { ($ name : ident :: Verbatim , $ member : ident) => { } ; ($ name : ident ::$ variant : ident , $ member : ident) => { impl From <$ member > for $ name { fn from (e : $ member) -> $ name { $ name ::$ variant (e) } } } ; }
    };
}

ast_enum_from_struct!()