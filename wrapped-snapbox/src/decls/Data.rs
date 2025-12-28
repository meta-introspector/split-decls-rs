macro_rules! deps {
    () => {
        DataInner!();
        DataSource!();
        FilterSet!();
    };
}

macro_rules! Data {
    () => {
        deps!();
        # [doc = " Test fixture, actual output, or expected result"] # [doc = ""] # [doc = " This provides conveniences for tracking the intended format (binary vs text)."] # [derive (Clone , Debug)] pub struct Data { pub (crate) inner : DataInner , pub (crate) source : Option < DataSource > , pub (crate) filters : FilterSet , }
    };
}

Data!();