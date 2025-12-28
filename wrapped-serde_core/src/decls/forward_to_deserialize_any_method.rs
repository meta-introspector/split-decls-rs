macro_rules! forward_to_deserialize_any_method {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! forward_to_deserialize_any_method { ($ func : ident <$ l : tt , $ v : ident > ($ ($ arg : ident : $ ty : ty) ,*)) => { # [inline] fn $ func <$ v > (self , $ ($ arg : $ ty ,) * visitor : $ v) -> $ crate :: __private :: Result <$ v :: Value , < Self as $ crate :: de :: Deserializer <$ l >>:: Error > where $ v : $ crate :: de :: Visitor <$ l >, { $ (let _ = $ arg ;) * self . deserialize_any (visitor) } } ; }
    };
}

forward_to_deserialize_any_method!();