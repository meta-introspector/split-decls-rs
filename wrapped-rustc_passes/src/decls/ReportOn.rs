macro_rules! ReportOn {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] enum ReportOn { # [doc = " Report on something that hasn't got a proper name to refer to"] TupleField , # [doc = " Report on something that has got a name, which could be a field but also a method"] NamedField , }
    };
}

ReportOn!();