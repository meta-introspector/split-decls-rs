macro_rules! deps {
    () => {
        InlineTable!();
        Value!();
        Item!();
        Table!();
        Index!();
        Key!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Index for str { fn index < 'v > (& self , v : & 'v Item) -> Option < & 'v Item > { match * v { Item :: Table (ref t) => t . get (self) , Item :: Value (ref v) => v . as_inline_table () . and_then (| t | t . items . get (self)) . and_then (| value | if ! value . is_none () { Some (value) } else { None }) , _ => None , } } fn index_mut < 'v > (& self , v : & 'v mut Item) -> Option < & 'v mut Item > { if let Item :: None = * v { let mut t = InlineTable :: default () ; t . items . insert (Key :: new (self) , Item :: None) ; * v = value (Value :: InlineTable (t)) ; } match * v { Item :: Table (ref mut t) => Some (t . entry (self) . or_insert (Item :: None)) , Item :: Value (ref mut v) => v . as_inline_table_mut () . map (| t | t . items . entry (Key :: new (self)) . or_insert_with (| | Item :: None)) , _ => None , } } }
    };
}

impl_77!()