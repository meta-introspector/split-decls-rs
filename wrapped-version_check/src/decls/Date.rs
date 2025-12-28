macro_rules! Date {
    () => {
        # [doc = " Release date including year, month, and day."] # [derive (Debug , PartialEq , Eq , Copy , Clone , PartialOrd , Ord)] pub struct Date (u32) ;
    };
}

Date!()