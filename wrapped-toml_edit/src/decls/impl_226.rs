macro_rules! deps {
    () => {
        KeyValuePairs!();
        Table!();
        InlineTable!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        # [doc = " Constructors"] # [doc = ""] # [doc = " See also `FromIterator`"] impl Table { # [doc = " Creates an empty table."] pub fn new () -> Self { Default :: default () } pub (crate) fn with_pos (doc_position : Option < isize >) -> Self { Self { doc_position , .. Default :: default () } } pub (crate) fn with_pairs (items : KeyValuePairs) -> Self { Self { items , .. Default :: default () } } # [doc = " Convert to an inline table"] pub fn into_inline_table (mut self) -> InlineTable { for (_ , value) in self . items . iter_mut () { value . make_value () ; } let mut t = InlineTable :: with_pairs (self . items) ; t . fmt () ; t } }
    };
}

impl_226!()