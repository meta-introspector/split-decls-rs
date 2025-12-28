macro_rules! deps {
    () => {
        InlineTable!();
        KeyValuePairs!();
        Table!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        # [doc = " Constructors"] # [doc = ""] # [doc = " See also `FromIterator`"] impl InlineTable { # [doc = " Creates an empty table."] pub fn new () -> Self { Default :: default () } pub (crate) fn with_pairs (items : KeyValuePairs) -> Self { Self { items , .. Default :: default () } } # [doc = " Convert to a table"] pub fn into_table (self) -> Table { let mut t = Table :: with_pairs (self . items) ; t . fmt () ; t } }
    };
}

impl_90!()