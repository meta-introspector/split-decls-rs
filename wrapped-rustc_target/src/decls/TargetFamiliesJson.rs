macro_rules! deps {
    () => {
        StaticCow!();
    };
}

macro_rules! TargetFamiliesJson {
    () => {
        deps!();
        # [derive (serde_derive :: Deserialize , schemars :: JsonSchema)] # [serde (untagged)] enum TargetFamiliesJson { Array (StaticCow < [StaticCow < str >] >) , String (StaticCow < str >) , }
    };
}

TargetFamiliesJson!()