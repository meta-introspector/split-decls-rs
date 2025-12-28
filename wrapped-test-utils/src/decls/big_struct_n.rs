macro_rules! big_struct_n {
    () => {
        pub fn big_struct_n (n : u32) -> String { let mut buf = "pub struct RegisterBlock {" . to_owned () ; for i in 0 .. n { format_to ! (buf , "  /// Doc comment for {}.\n" , i) ; format_to ! (buf , "  pub s{}: S{},\n" , i , i) ; } buf . push_str ("}\n\n") ; for i in 0 .. n { format_to ! (buf , "

#[repr(transparent)]
struct S{} {{
    field: u32,
}}" , i) ; } buf }
    };
}

big_struct_n!();