macro_rules! ALLOWED_RETURN_MODES {
    () => {
        const ALLOWED_RETURN_MODES : & [& str] = & ["copy" , "clone" , "ref" , "deref" , "as_ref" , "as_deref"] ;
    };
}

ALLOWED_RETURN_MODES!();