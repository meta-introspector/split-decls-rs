macro_rules! deps {
    () => {
        StepCursor!();
        Cursor!();
    };
}

macro_rules! advance_step_cursor {
    () => {
        deps!();
        pub (crate) fn advance_step_cursor < 'c , 'a > (proof : StepCursor < 'c , 'a > , to : Cursor < 'c >) -> Cursor < 'a > { let _ = proof ; unsafe { mem :: transmute :: < Cursor < 'c > , Cursor < 'a > > (to) } }
    };
}

advance_step_cursor!()