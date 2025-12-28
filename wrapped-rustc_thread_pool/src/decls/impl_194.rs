macro_rules! deps {
    () => {
        ScopePtr!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        unsafe impl < T : Sync > Sync for ScopePtr < T > { }
    };
}

impl_194!()