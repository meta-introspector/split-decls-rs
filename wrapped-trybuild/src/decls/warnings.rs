macro_rules! warnings {
    () => {
        pub (crate) fn warnings (warnings : & str) { if warnings . is_empty () { return ; } term :: bold_color (Yellow) ; println ! ("WARNINGS:") ; snippet (Yellow , warnings) ; println ! () ; }
    };
}

warnings!();