macro_rules! deps {
    () => {
        Table!();
        Buffer!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl Buffer { # [doc = " Initialize a new serialization buffer"] pub fn new () -> Self { Default :: default () } # [doc = " Reset the buffer for serializing another document"] pub fn clear (& mut self) { self . tables . clear () ; } pub (crate) fn root_table (& mut self) -> Table { self . new_table (None) } pub (crate) fn child_table (& mut self , parent : & mut Table , key : String) -> Table { parent . has_children = true ; let mut key_path = parent . key . clone () ; key_path . get_or_insert_with (Vec :: new) . push (key) ; self . new_table (key_path) } pub (crate) fn element_table (& mut self , parent : & mut Table , key : String) -> Table { let mut table = self . child_table (parent , key) ; table . array = true ; table } pub (crate) fn new_table (& mut self , key : Option < Vec < String > >) -> Table { let pos = self . tables . len () ; let table = Table { key , body : String :: new () , has_children : false , pos , array : false , } ; self . tables . push (None) ; table } pub (crate) fn push (& mut self , table : Table) { let pos = table . pos ; self . tables [pos] = Some (table) ; } }
    };
}

impl_279!()