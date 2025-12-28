macro_rules! TimingGuard {
    () => {
        # [must_use] pub struct TimingGuard < 'a > (Option < measureme :: TimingGuard < 'a > >) ;
    };
}

TimingGuard!();