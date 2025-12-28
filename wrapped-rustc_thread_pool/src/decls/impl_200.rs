macro_rules! deps {
    () => {
        JobsEventCounter!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl JobsEventCounter { pub (super) const DUMMY : JobsEventCounter = JobsEventCounter (usize :: MAX) ; # [inline] pub (super) fn as_usize (self) -> usize { self . 0 } # [doc = " The JEC \"is sleepy\" if the last thread to increment it was in the"] # [doc = " process of becoming sleepy. This is indicated by its value being *even*."] # [doc = " When new jobs are posted, they check if the JEC is sleepy, and if so"] # [doc = " they incremented it."] # [inline] pub (super) fn is_sleepy (self) -> bool { (self . as_usize () & 1) == 0 } # [doc = " The JEC \"is active\" if the last thread to increment it was posting new"] # [doc = " work. This is indicated by its value being *odd*. When threads get"] # [doc = " sleepy, they will check if the JEC is active, and increment it."] # [inline] pub (super) fn is_active (self) -> bool { ! self . is_sleepy () } }
    };
}

impl_200!()