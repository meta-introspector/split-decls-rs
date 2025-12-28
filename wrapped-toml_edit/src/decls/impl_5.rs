macro_rules! deps {
    () => {
        Array!();
        Decor!();
        RawString!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [doc = " Formatting"] impl Array { # [doc = " Auto formats the array."] pub fn fmt (& mut self) { decorate_array (self) ; } # [doc = " Set whether the array will use a trailing comma"] pub fn set_trailing_comma (& mut self , yes : bool) { self . trailing_comma = yes ; } # [doc = " Whether the array will use a trailing comma"] pub fn trailing_comma (& self) -> bool { self . trailing_comma } # [doc = " Set whitespace after last element"] pub fn set_trailing (& mut self , trailing : impl Into < RawString >) { self . trailing = trailing . into () ; } # [doc = " Whitespace after last element"] pub fn trailing (& self) -> & RawString { & self . trailing } # [doc = " Returns the surrounding whitespace"] pub fn decor_mut (& mut self) -> & mut Decor { & mut self . decor } # [doc = " Returns the surrounding whitespace"] pub fn decor (& self) -> & Decor { & self . decor } # [doc = " The location within the original document"] # [doc = ""] # [doc = " This generally requires a [`Document`][crate::Document]."] pub fn span (& self) -> Option < std :: ops :: Range < usize > > { self . span . clone () } pub (crate) fn despan (& mut self , input : & str) { self . span = None ; self . decor . despan (input) ; self . trailing . despan (input) ; for value in & mut self . values { value . despan (input) ; } } }
    };
}

impl_5!()