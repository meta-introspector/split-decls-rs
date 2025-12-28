macro_rules! deps {
    () => {
        Item!();
        Table!();
        Iter!();
        RawString!();
        Document!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < S > Document < S > { # [doc = " Returns a reference to the root item."] pub fn as_item (& self) -> & Item { & self . root } # [doc = " Returns the root item."] pub fn into_item (self) -> Item { self . root } # [doc = " Returns a reference to the root table."] pub fn as_table (& self) -> & Table { self . root . as_table () . expect ("root should always be a table") } # [doc = " Returns the root table."] pub fn into_table (self) -> Table { self . root . into_table () . expect ("root should always be a table") } # [doc = " Returns an iterator over the root table."] pub fn iter (& self) -> Iter < '_ > { self . as_table () . iter () } # [doc = " Whitespace after last element"] pub fn trailing (& self) -> & RawString { & self . trailing } }
    };
}

impl_31!();