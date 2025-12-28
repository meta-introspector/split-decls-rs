macro_rules! UnhandledPanic {
    () => {
        # [derive (Clone , Copy , PartialEq)] enum UnhandledPanic { Ignore , ShutdownRuntime , }
    };
}

UnhandledPanic!()