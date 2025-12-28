macro_rules! NULL {
    () => {
        const NULL : * mut c_void = null_mut () ;
    };
}

NULL!()