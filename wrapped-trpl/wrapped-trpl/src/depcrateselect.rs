// Generated macro for select (function)
macro_rules! Depcrateselect {
() => {
// Module: crate
// Provides: {"select"}
// Dependencies: {}
# [doc = " Run two futures, taking whichever finishes first and canceling the other."] # [doc = ""] # [doc = " Notice that this is built on [`futures::future::select`], which has the"] # [doc = " same overall semantics but does *not* drop the slower future. The idea there"] # [doc = " is that you can work with the first result and then later *also* continue"] # [doc = " waiting for the second future."] # [doc = ""] # [doc = " We drop the slower future for the sake of simplicity in the examples: no"] # [doc = " need to deal with the tuple and intentionally ignore the second future this"] # [doc = " way!"] # [doc = ""] # [doc = " Note that this only works as “simply” as it does because:"] # [doc = ""] # [doc = " - It takes ownership of the futures."] # [doc = " - It internally *pins* the futures."] # [doc = " - It throws away (rather than returning) the unused future (which is why it"] # [doc = "   can get away with pinning them)."] pub async fn select < A , B , F1 , F2 > (f1 : F1 , f2 : F2) -> Either < A , B > where F1 : Future < Output = A > , F2 : Future < Output = B > , { let f1 = pin ! (f1) ; let f2 = pin ! (f2) ; match future :: select (f1 , f2) . await { Either :: Left ((a , _f2)) => Either :: Left (a) , Either :: Right ((b , _f1)) => Either :: Right (b) , } }
};
}
