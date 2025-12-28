macro_rules! Lifetime {
    () => {
        # [doc = " A Rust lifetime: `'a`."] # [doc = ""] # [doc = " Lifetime names must conform to the following rules:"] # [doc = ""] # [doc = " - Must start with an apostrophe."] # [doc = " - Must not consist of just an apostrophe: `'`."] # [doc = " - Character after the apostrophe must be `_` or a Unicode code point with"] # [doc = "   the XID_Start property."] # [doc = " - All following characters must be Unicode code points with the XID_Continue"] # [doc = "   property."] pub struct Lifetime { pub apostrophe : Span , pub ident : Ident , }
    };
}

Lifetime!();