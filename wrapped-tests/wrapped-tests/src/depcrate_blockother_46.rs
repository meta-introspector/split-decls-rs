// Generated macro for other_46 (other)
macro_rules! Depcrate_blockother_46 {
() => {
// Module: crate::block
// Provides: {"other_46"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns a pointer to a global block that returns 7."] fn get_int_block () -> * mut Block < dyn Fn () -> i32 > ; # [doc = " Returns a pointer to a copied block that returns `i`."] fn get_int_block_with (i : i32) -> * mut Block < dyn Fn () -> i32 > ; # [doc = " Invokes a block and returns its result."] fn invoke_int_block (block : & Block < dyn Fn () -> i32 >) -> i32 ; # [doc = " Returns a pointer to a global block that returns its argument + 7."] fn get_add_block () -> * mut Block < dyn Fn (i32) -> i32 > ; # [doc = " Returns a pointer to a copied block that returns its argument + `i`."] fn get_add_block_with (i : i32) -> * mut Block < dyn Fn (i32) -> i32 > ; # [doc = " Invokes a block with `a` and returns the result."] fn invoke_add_block (block : & Block < dyn Fn (i32) -> i32 > , a : i32) -> i32 ; fn get_add_12 () -> * mut Add12 ; fn get_add_12_with (x : i32) -> * mut Add12 ; fn invoke_add_12 (block : & Add12 , a1 : i32 , a2 : i32 , a3 : i32 , a4 : i32 , a5 : i32 , a6 : i32 , a7 : i32 , a8 : i32 , a9 : i32 , a10 : i32 , a11 : i32 , a12 : i32 ,) -> i32 ; fn get_large_struct_block () -> * mut Block < dyn Fn (LargeStruct) -> LargeStruct > ; fn get_large_struct_block_with (i : LargeStruct ,) -> * mut Block < dyn Fn (LargeStruct) -> LargeStruct > ; fn invoke_large_struct_block (block : & Block < dyn Fn (LargeStruct) -> LargeStruct > , s : LargeStruct ,) -> LargeStruct ; fn try_block_debugging (x : i32) ; }
};
}
