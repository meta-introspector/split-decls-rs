macro_rules! deps {
    () => {
        ArrayOfTables!();
        Array!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [doc = " Formatting"] impl ArrayOfTables { # [doc = " Convert to an inline array"] pub fn into_array (mut self) -> Array { for value in self . values . iter_mut () { value . make_value () ; } let mut a = Array :: with_vec (self . values) ; a . fmt () ; a } # [doc = " The location within the original document"] # [doc = ""] # [doc = " This generally requires a [`Document`][crate::Document]."] pub fn span (& self) -> Option < std :: ops :: Range < usize > > { self . span . clone () } pub (crate) fn despan (& mut self , input : & str) { self . span = None ; for value in & mut self . values { value . despan (input) ; } } }
    };
}

impl_16!()