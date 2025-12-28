macro_rules! ParamEnvSource {
    () => {
        # [derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] pub enum ParamEnvSource { # [doc = " Preferred eagerly."] NonGlobal , Global , }
    };
}

ParamEnvSource!()