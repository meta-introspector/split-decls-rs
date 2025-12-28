macro_rules! deps {
    () => {
        StaticCow!();
    };
}

macro_rules! SmallDataThresholdSupport {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Hash)] pub enum SmallDataThresholdSupport { None , DefaultForArch , LlvmModuleFlag (StaticCow < str >) , LlvmArg (StaticCow < str >) , }
    };
}

SmallDataThresholdSupport!()