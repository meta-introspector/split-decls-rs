macro_rules! failed_to_build {
    () => {
        pub (crate) fn failed_to_build (stderr : & str) { term :: bold_color (Red) ; println ! ("error") ; snippet (Red , stderr) ; println ! () ; }
    };
}

failed_to_build!()