macro_rules! deps {
    () => {
        DocumentMut!();
        Iter!();
        Table!();
        RawString!();
        Item!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl DocumentMut { # [doc = " Creates an empty document"] pub fn new () -> Self { Default :: default () } # [doc = " Returns a reference to the root item."] pub fn as_item (& self) -> & Item { & self . root } # [doc = " Returns a mutable reference to the root item."] pub fn as_item_mut (& mut self) -> & mut Item { & mut self . root } # [doc = " Returns the root item."] pub fn into_item (self) -> Item { self . root } # [doc = " Returns a reference to the root table."] pub fn as_table (& self) -> & Table { self . root . as_table () . expect ("root should always be a table") } # [doc = " Returns a mutable reference to the root table."] pub fn as_table_mut (& mut self) -> & mut Table { self . root . as_table_mut () . expect ("root should always be a table") } # [doc = " Returns the root table."] pub fn into_table (self) -> Table { self . root . into_table () . expect ("root should always be a table") } # [doc = " Returns an iterator over the root table."] pub fn iter (& self) -> Iter < '_ > { self . as_table () . iter () } # [doc = " Set whitespace after last element"] pub fn set_trailing (& mut self , trailing : impl Into < RawString >) { self . trailing = trailing . into () ; } # [doc = " Whitespace after last element"] pub fn trailing (& self) -> & RawString { & self . trailing } }
    };
}

impl_38!();