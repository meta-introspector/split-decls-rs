macro_rules! ACCESS_ERROR {
    () => {
        const ACCESS_ERROR : & str = "jobserver check should have been called earlier" ;
    };
}

ACCESS_ERROR!()