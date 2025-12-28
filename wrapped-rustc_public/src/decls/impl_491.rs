macro_rules! deps {
    () => {
        Span!();
        Ty!();
        Error!();
        ItemKind!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl CrateItem { # [doc = " This will return the body of an item or panic if it's not available."] pub fn expect_body (& self) -> mir :: Body { with (| cx | cx . mir_body (self . 0)) } # [doc = " Return the body of an item if available."] pub fn body (& self) -> Option < mir :: Body > { with (| cx | cx . has_body (self . 0) . then (| | cx . mir_body (self . 0))) } # [doc = " Check if a body is available for this item."] pub fn has_body (& self) -> bool { with (| cx | cx . has_body (self . 0)) } pub fn span (& self) -> Span { with (| cx | cx . span_of_an_item (self . 0)) } pub fn kind (& self) -> ItemKind { with (| cx | cx . item_kind (* self)) } pub fn requires_monomorphization (& self) -> bool { with (| cx | cx . requires_monomorphization (self . 0)) } pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . 0)) } pub fn is_foreign_item (& self) -> bool { with (| cx | cx . is_foreign_item (self . 0)) } # [doc = " Emit MIR for this item body."] pub fn emit_mir < W : io :: Write > (& self , w : & mut W) -> io :: Result < () > { self . body () . ok_or_else (| | io :: Error :: other (format ! ("No body found for `{}`" , self . name ()))) ? . dump (w , & self . name ()) } }
    };
}

impl_491!();