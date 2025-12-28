macro_rules! deps {
    () => {
        PWSTR!();
        LPSERVICE_MAIN_FUNCTIONW!();
    };
}

macro_rules! SERVICE_TABLE_ENTRYW {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct SERVICE_TABLE_ENTRYW { pub lpServiceName : PWSTR , pub lpServiceProc : LPSERVICE_MAIN_FUNCTIONW , }
    };
}

SERVICE_TABLE_ENTRYW!();