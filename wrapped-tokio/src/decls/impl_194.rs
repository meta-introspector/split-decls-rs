macro_rules! deps {
    () => {
        AtomicUsize!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        unsafe impl Send for AtomicUsize { }
    };
}

impl_194!()