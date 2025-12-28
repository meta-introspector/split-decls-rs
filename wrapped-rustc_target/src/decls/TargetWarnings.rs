macro_rules! TargetWarnings {
    () => {
        # [doc = " Warnings encountered when parsing the target `json`."] # [doc = ""] # [doc = " Includes fields that weren't recognized and fields that don't have the expected type."] # [derive (Debug , PartialEq)] pub struct TargetWarnings { unused_fields : Vec < String > , }
    };
}

TargetWarnings!()