macro_rules! deps {
    () => {
        Dispatch!();
    };
}

macro_rules! dispatchers {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] mod dispatchers { use crate :: dispatcher ; pub (super) struct Dispatchers (()) ; pub (super) struct Rebuilder < 'a > (Option < & 'a dispatcher :: Dispatch >) ; impl Dispatchers { pub (super) const fn new () -> Self { Self (()) } pub (super) fn rebuilder (& self) -> Rebuilder < '_ > { Rebuilder (None) } pub (super) fn register_dispatch < 'dispatch > (& self , dispatch : & 'dispatch dispatcher :: Dispatch ,) -> Rebuilder < 'dispatch > { Rebuilder (Some (dispatch)) } } impl Rebuilder < '_ > { # [inline] pub (super) fn for_each (& self , mut f : impl FnMut (& dispatcher :: Dispatch)) { if let Some (dispatch) = self . 0 { f (dispatch) } else { dispatcher :: get_default (f) } } } }
    };
}

dispatchers!()