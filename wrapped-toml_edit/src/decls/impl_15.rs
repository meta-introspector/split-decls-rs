macro_rules! deps {
    () => {
        ArrayOfTables!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Constructors"] # [doc = ""] # [doc = " See also `FromIterator`"] impl ArrayOfTables { # [doc = " Creates an empty array of tables."] pub fn new () -> Self { Default :: default () } }
    };
}

impl_15!()