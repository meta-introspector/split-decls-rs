macro_rules! FormatExtendType {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Debug)] enum FormatExtendType { AcceptAny , AcceptNone , RequireLetter , RequireHLetter , AcceptQLetter , RequireNumeric , }
    };
}

FormatExtendType!();