macro_rules! deps {
    () => {
        Uniform!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl Uniform { pub fn align < C : HasDataLayout > (& self , cx : & C) -> Align { self . unit . align (cx) } # [doc = " Pass using one or more values of the given type, without requiring them to be consecutive."] # [doc = " That is, some values may be passed in register and some on the stack."] pub fn new (unit : Reg , total : Size) -> Self { Uniform { unit , total , is_consecutive : false } } # [doc = " Pass using one or more consecutive values of the given type. Either all values will be"] # [doc = " passed in registers, or all on the stack."] pub fn consecutive (unit : Reg , total : Size) -> Self { Uniform { unit , total , is_consecutive : true } } }
    };
}

impl_276!()