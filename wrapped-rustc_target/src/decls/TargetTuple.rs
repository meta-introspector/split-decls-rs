macro_rules! TargetTuple {
    () => {
        # [doc = " Either a target tuple string or a path to a JSON file."] # [derive (Clone , Debug)] pub enum TargetTuple { TargetTuple (String) , TargetJson { # [doc = " Warning: This field may only be used by rustdoc. Using it anywhere else will lead to"] # [doc = " inconsistencies as it is discarded during serialization."] path_for_rustdoc : PathBuf , tuple : String , contents : String , } , }
    };
}

TargetTuple!();