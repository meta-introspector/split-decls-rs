macro_rules! deps {
    () => {
        DefId!();
        Symbol!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl DefId { # [doc = " Return fully qualified name of this definition"] pub fn name (& self) -> Symbol { with (| cx | cx . def_name (* self , false)) } # [doc = " Return a trimmed name of this definition."] # [doc = ""] # [doc = " This can be used to print more user friendly diagnostic messages."] # [doc = ""] # [doc = " If a symbol name can only be imported from one place for a type, and as"] # [doc = " long as it was not glob-imported anywhere in the current crate, we trim its"] # [doc = " path and print only the name."] # [doc = ""] # [doc = " For example, this function may shorten `std::vec::Vec` to just `Vec`,"] # [doc = " as long as there is no other `Vec` importable anywhere."] pub fn trimmed_name (& self) -> Symbol { with (| cx | cx . def_name (* self , true)) } }
    };
}

impl_252!()