macro_rules! UtcDateTime {
    () => {
        # [doc = " Retrieve and print the current wall-clock time in UTC timezone."] # [cfg (feature = "time")] # [derive (Debug , Clone , Copy , Eq , PartialEq , Default)] pub struct UtcDateTime { # [doc = " Whether to print the time with higher precision."] pub higher_precision : bool , }
    };
}

UtcDateTime!();