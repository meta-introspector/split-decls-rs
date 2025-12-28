macro_rules! deps {
    () => {
        Ty!();
        FieldDef!();
        GenericArgs!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl FieldDef { # [doc = " Retrieve the type of this field instantiating and normalizing it with the given arguments."] # [doc = ""] # [doc = " This will assume the type can be instantiated with these arguments."] pub fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . def , args)) } # [doc = " Retrieve the type of this field."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . def)) } }
    };
}

impl_363!();