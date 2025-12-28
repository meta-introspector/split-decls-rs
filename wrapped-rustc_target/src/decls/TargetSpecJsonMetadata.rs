macro_rules! deps {
    () => {
        StaticCow!();
    };
}

macro_rules! TargetSpecJsonMetadata {
    () => {
        deps!();
        # [derive (serde_derive :: Deserialize , schemars :: JsonSchema)] struct TargetSpecJsonMetadata { description : Option < StaticCow < str > > , tier : Option < u64 > , host_tools : Option < bool > , std : Option < bool > , }
    };
}

TargetSpecJsonMetadata!()