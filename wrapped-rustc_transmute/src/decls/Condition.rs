macro_rules! Condition {
    () => {
        # [doc = " A condition which must hold for safe transmutation to be possible."] # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub enum Condition < R , T > { # [doc = " `Src` is transmutable into `Dst`, if `src` is transmutable into `dst`."] Transmutable { src : T , dst : T } , # [doc = " The region `long` must outlive `short`."] Outlives { long : R , short : R } , # [doc = " The `ty` is immutable."] Immutable { ty : T } , # [doc = " `Src` is transmutable into `Dst`, if all of the enclosed requirements are met."] IfAll (Vec < Condition < R , T > >) , # [doc = " `Src` is transmutable into `Dst` if any of the enclosed requirements are met."] IfAny (Vec < Condition < R , T > >) , }
    };
}

Condition!();