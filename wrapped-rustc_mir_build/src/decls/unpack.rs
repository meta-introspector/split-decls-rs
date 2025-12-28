macro_rules! deps {
    () => {
        BlockAnd!();
    };
}

macro_rules! unpack {
    () => {
        deps!();
        # [doc = " Update a block pointer and return the value."] # [doc = " Use it like `let x = unpack!(block = self.foo(block, foo))`."] macro_rules ! unpack { ($ x : ident = $ c : expr) => { { let BlockAnd (b , v) = $ c ; $ x = b ; v } } ; }
    };
}

unpack!()