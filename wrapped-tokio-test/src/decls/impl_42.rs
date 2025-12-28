macro_rules! deps {
    () => {
        Spawn!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T > Spawn < T > { # [doc = " Consumes `self` returning the inner value"] pub fn into_inner (self) -> T where T : Unpin , { * Pin :: into_inner (self . future) } # [doc = " Returns `true` if the inner future has received a wake notification"] # [doc = " since the last call to `enter`."] pub fn is_woken (& self) -> bool { self . task . is_woken () } # [doc = " Returns the number of references to the task waker"] # [doc = ""] # [doc = " The task itself holds a reference. The return value will never be zero."] pub fn waker_ref_count (& self) -> usize { self . task . waker_ref_count () } # [doc = " Enter the task context"] pub fn enter < F , R > (& mut self , f : F) -> R where F : FnOnce (& mut Context < '_ > , Pin < & mut T >) -> R , { let fut = self . future . as_mut () ; self . task . enter (| cx | f (cx , fut)) } }
    };
}

impl_42!()