macro_rules! deps {
    () => {
        MacroResult!();
        Structure!();
    };
}

macro_rules! test_derive {
    () => {
        deps!();
        # [doc = " Run a test on a custom derive. This macro expands both the original struct"] # [doc = " and the expansion to ensure that they compile correctly, and confirms that"] # [doc = " feeding the original struct into the named derive will produce the written"] # [doc = " output."] # [doc = ""] # [doc = " You can add `no_build` to the end of the macro invocation to disable"] # [doc = " checking that the written code compiles. This is useful in contexts where"] # [doc = " the procedural macro cannot depend on the crate where it is used during"] # [doc = " tests."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```"] # [doc = " fn test_derive_example(_s: synstructure::Structure)"] # [doc = "     -> Result<proc_macro2::TokenStream, syn::Error>"] # [doc = " {"] # [doc = "     Ok(quote::quote! { const YOUR_OUTPUT: &'static str = \"here\"; })"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     synstructure::test_derive!{"] # [doc = "         test_derive_example {"] # [doc = "             struct A;"] # [doc = "         }"] # [doc = "         expands to {"] # [doc = "             const YOUR_OUTPUT: &'static str = \"here\";"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! test_derive { ($ name : path { $ ($ i : tt) * } expands to { $ ($ o : tt) * }) => { { # [allow (dead_code)] fn ensure_compiles () { $ ($ i) * $ ($ o) * } $ crate :: test_derive ! ($ name { $ ($ i) * } expands to { $ ($ o) * } no_build) ; } } ; ($ name : path { $ ($ i : tt) * } expands to { $ ($ o : tt) * } no_build) => { { let i = $ crate :: macros :: quote ! ($ ($ i) *) ; let parsed = $ crate :: macros :: parse2 ::<$ crate :: macros :: DeriveInput > (i) . expect (:: core :: concat ! ("Failed to parse input to `#[derive(" , :: core :: stringify ! ($ name) , ")]`" ,)) ; let raw_res = $ name ($ crate :: Structure :: new (& parsed)) ; let res = $ crate :: MacroResult :: into_result (raw_res) . expect (:: core :: concat ! ("Procedural macro failed for `#[derive(" , :: core :: stringify ! ($ name) , ")]`" ,)) ; let expected_toks = $ crate :: macros :: quote ! ($ ($ o) *) ; if <$ crate :: macros :: TokenStream2 as :: std :: string :: ToString >:: to_string (& res) != <$ crate :: macros :: TokenStream2 as :: std :: string :: ToString >:: to_string (& expected_toks) { panic ! ("\
test_derive failed:
expected:
```
{}
```

got:
```
{}
```\n" , $ crate :: unpretty_print (& expected_toks) , $ crate :: unpretty_print (& res) ,) ; } } } ; }
    };
}

test_derive!()