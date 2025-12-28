macro_rules! deps {
    () => {
        RevisionQueue!();
        Revision!();
        Configuration!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < C : Configuration > RevisionQueue < C > { # [doc = " Record the given revision as active."] # [inline] fn record (& self , revision : Revision) { if C :: REVISIONS == IMMORTAL { return ; } if self . revisions [0] . load () >= revision { return ; } self . record_cold (revision) ; } # [cold] fn record_cold (& self , revision : Revision) { let _lock = self . lock . lock () ; for i in (1 .. C :: REVISIONS . get ()) . rev () { self . revisions [i] . store (self . revisions [i - 1] . load ()) ; } self . revisions [0] . store (revision) ; } # [doc = " Returns `true` if the given revision is old enough to be considered stale."] # [inline] fn is_stale (& self , revision : Revision) -> bool { if C :: REVISIONS == IMMORTAL { return false ; } let oldest = self . revisions [C :: REVISIONS . get () - 1] . load () ; if oldest == Revision :: start () { return false ; } revision < oldest } # [doc = " Returns `true` if `C::REVISIONS` revisions have been recorded as active,"] # [doc = " i.e. enough data has been recorded to start garbage collection."] # [inline] fn is_primed (& self) -> bool { if C :: REVISIONS == IMMORTAL { return false ; } self . revisions [C :: REVISIONS . get () - 1] . load () > Revision :: start () } }
    };
}

impl_198!();