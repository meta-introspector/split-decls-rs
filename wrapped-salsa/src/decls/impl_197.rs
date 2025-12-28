macro_rules! deps {
    () => {
        AtomicRevision!();
        RevisionQueue!();
        Configuration!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < C : Configuration > Default for RevisionQueue < C > { fn default () -> RevisionQueue < C > { let revisions = if C :: REVISIONS == IMMORTAL { Box :: default () } else { (0 .. C :: REVISIONS . get ()) . map (| _ | AtomicRevision :: start ()) . collect () } ; RevisionQueue { lock : Mutex :: new (()) , revisions , _configuration : PhantomData , } } }
    };
}

impl_197!();