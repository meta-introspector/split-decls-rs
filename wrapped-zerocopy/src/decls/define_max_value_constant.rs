macro_rules! define_max_value_constant {
    () => {
        macro_rules ! define_max_value_constant { ($ name : ident , $ bytes : expr , "unsigned integer") => { # [doc = " The maximum value."] # [doc = ""] # [doc = " This constant should be preferred to constructing a new value using"] # [doc = " `new`, as `new` may perform an endianness swap depending on the"] # [doc = " endianness `O` and the endianness of the platform."] pub const MAX_VALUE : $ name < O > = $ name ([0xFFu8 ; $ bytes] , PhantomData) ; } ; ($ name : ident , $ bytes : expr , "signed integer") => { } ; ($ name : ident , $ bytes : expr , "floating point number") => { } ; }
    };
}

define_max_value_constant!();