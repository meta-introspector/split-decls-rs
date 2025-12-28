macro_rules! deps {
    () => {
        MoveData!();
    };
}

macro_rules! EverInitializedPlaces {
    () => {
        deps!();
        # [doc = " `EverInitializedPlaces` tracks all places that might have ever been"] # [doc = " initialized upon reaching a particular point in the control flow"] # [doc = " for a function, without an intervening `StorageDead`."] # [doc = ""] # [doc = " This dataflow is used to determine if an immutable local variable may"] # [doc = " be assigned to."] # [doc = ""] # [doc = " For example, in code like the following, we have corresponding"] # [doc = " dataflow information shown in the right-hand comments."] # [doc = ""] # [doc = " ```rust"] # [doc = " struct S;"] # [doc = " #[rustfmt::skip]"] # [doc = " fn foo(pred: bool) {                        // ever-init:"] # [doc = "                                             // {          }"] # [doc = "     let a = S; let mut b = S; let c; let d; // {a, b      }"] # [doc = ""] # [doc = "     if pred {"] # [doc = "         drop(a);                            // {a, b,     }"] # [doc = "         b = S;                              // {a, b,     }"] # [doc = ""] # [doc = "     } else {"] # [doc = "         drop(b);                            // {a, b,      }"] # [doc = "         d = S;                              // {a, b,    d }"] # [doc = ""] # [doc = "     }                                       // {a, b,    d }"] # [doc = ""] # [doc = "     c = S;                                  // {a, b, c, d }"] # [doc = " }"] # [doc = " ```"] pub struct EverInitializedPlaces < 'a , 'tcx > { body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx > , }
    };
}

EverInitializedPlaces!()