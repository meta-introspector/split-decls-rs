macro_rules! deps {
    () => {
        MakeVisitor!();
        Messages!();
        Alt!();
        Delimited!();
        MakeExtMarker!();
        VisitFmt!();
    };
}

macro_rules! MakeExt {
    () => {
        deps!();
        # [doc = " Extension trait providing `MakeVisitor` combinators."] pub trait MakeExt < T > where Self : MakeVisitor < T > + Sized , Self : crate :: sealed :: Sealed < MakeExtMarker < T > > , { # [doc = " Wraps `self` so that any `fmt::Debug` fields are recorded using the"] # [doc = " alternate formatter (`{:#?}`)."] fn debug_alt (self) -> debug :: Alt < Self > { debug :: Alt :: new (self) } # [doc = " Wraps `self` so that any string fields named \"message\" are recorded"] # [doc = " using `fmt::Display`."] fn display_messages (self) -> display :: Messages < Self > { display :: Messages :: new (self) } # [doc = " Wraps `self` so that when fields are formatted to a writer, they are"] # [doc = " separated by the provided `delimiter`."] fn delimited < D > (self , delimiter : D) -> delimited :: Delimited < D , Self > where D : AsRef < str > + Clone , Self :: Visitor : VisitFmt , { delimited :: Delimited :: new (delimiter , self) } }
    };
}

MakeExt!()