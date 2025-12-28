macro_rules! deps {
    () => {
        FilterSet!();
        DataFormat!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl FilterSet { pub (crate) fn new () -> Self { Self :: empty () . redactions () . newlines () . paths () } pub (crate) const fn empty () -> Self { Self { flags : 0 , against : None , } } pub (crate) fn redactions (mut self) -> Self { self . set (Self :: REDACTIONS) ; self } pub (crate) fn newlines (mut self) -> Self { self . set (Self :: NEWLINES) ; self } pub (crate) fn paths (mut self) -> Self { self . set (Self :: PATHS) ; self } pub (crate) fn unordered (mut self) -> Self { self . set (Self :: UNORDERED) ; self } pub (crate) fn against (mut self , format : DataFormat) -> Self { self . against = Some (format) ; self } pub (crate) const fn is_redaction_set (& self) -> bool { self . is_set (Self :: REDACTIONS) } pub (crate) const fn is_newlines_set (& self) -> bool { self . is_set (Self :: NEWLINES) } pub (crate) const fn is_paths_set (& self) -> bool { self . is_set (Self :: PATHS) } pub (crate) const fn is_unordered_set (& self) -> bool { self . is_set (Self :: UNORDERED) } pub (crate) const fn get_against (& self) -> Option < DataFormat > { self . against } }
    };
}

impl_56!()