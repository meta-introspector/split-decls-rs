macro_rules! deps {
    () => {
        PWSTR!();
    };
}

macro_rules! LPSERVICE_MAIN_FUNCTIONW {
    () => {
        deps!();
        pub type LPSERVICE_MAIN_FUNCTIONW = Option < unsafe extern "system" fn (dwnumservicesargs : u32 , lpserviceargvectors : * mut PWSTR) > ;
    };
}

LPSERVICE_MAIN_FUNCTIONW!();