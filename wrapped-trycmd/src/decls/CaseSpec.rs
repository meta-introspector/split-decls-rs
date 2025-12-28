macro_rules! deps {
    () => {
        CommandStatus!();
    };
}

macro_rules! CaseSpec {
    () => {
        deps!();
        # [derive (Debug)] struct CaseSpec { glob : std :: path :: PathBuf , expected : Option < crate :: schema :: CommandStatus > , }
    };
}

CaseSpec!()