macro_rules! deps {
    () => {
        SolverDelegate!();
    };
}

macro_rules! EagerResolver {
    () => {
        deps!();
        # [doc = " Resolves ty, region, and const vars to their inferred values or their root vars."] struct EagerResolver < 'a , D , I = < D as SolverDelegate > :: Interner > where D : SolverDelegate < Interner = I > , I : Interner , { delegate : & 'a D , # [doc = " We're able to use a cache here as the folder does not have any"] # [doc = " mutable state."] cache : DelayedMap < I :: Ty , I :: Ty > , }
    };
}

EagerResolver!();