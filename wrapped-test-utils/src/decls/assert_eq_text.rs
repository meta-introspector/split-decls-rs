macro_rules! assert_eq_text {
    () => {
        # [doc = " Asserts that two strings are equal, otherwise displays a rich diff between them."] # [doc = ""] # [doc = " The diff shows changes from the \"original\" left string to the \"actual\" right string."] # [doc = ""] # [doc = " All arguments starting from and including the 3rd one are passed to"] # [doc = " `eprintln!()` macro in case of text inequality."] # [macro_export] macro_rules ! assert_eq_text { ($ left : expr , $ right : expr) => { $ crate :: assert_eq_text ! ($ left , $ right ,) } ; ($ left : expr , $ right : expr , $ ($ tt : tt) *) => { { let left = $ left ; let right = $ right ; if left != right { if left . trim () == right . trim () { std :: eprintln ! ("Left:\n{:?}\n\nRight:\n{:?}\n\nWhitespace difference\n" , left , right) ; } else { let diff = $ crate :: __diff (left , right) ; std :: eprintln ! ("Left:\n{}\n\nRight:\n{}\n\nDiff:\n{}\n" , left , right , $ crate :: format_diff (diff)) ; } std :: eprintln ! ($ ($ tt) *) ; panic ! ("text differs") ; } } } ; }
    };
}

assert_eq_text!()