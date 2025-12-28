macro_rules! ElaborateSized {
    () => {
        # [derive (Eq , PartialEq)] enum ElaborateSized { Yes , No , }
    };
}

ElaborateSized!()