// Generated macro for unpack (macro)
macro_rules! Depcrate_builderunpack {
() => {
// Module: crate::builder
// Provides: {"unpack"}
// Dependencies: {}
# [doc = " Update a block pointer and return the value."] # [doc = " Use it like `let x = unpack!(block = self.foo(block, foo))`."] macro_rules ! unpack { ($ x : ident = $ c : expr) => { { let BlockAnd (b , v) = $ c ; $ x = b ; v } } ; }
};
}
