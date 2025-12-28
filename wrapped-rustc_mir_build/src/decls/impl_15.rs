macro_rules! deps {
    () => {
        LocalsForNode!();
        ForGuard!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl LocalsForNode { fn local_id (& self , for_guard : ForGuard) -> Local { match (self , for_guard) { (& LocalsForNode :: One (local_id) , ForGuard :: OutsideGuard) | (& LocalsForNode :: ForGuard { ref_for_guard : local_id , .. } , ForGuard :: RefWithinGuard ,) | (& LocalsForNode :: ForGuard { for_arm_body : local_id , .. } , ForGuard :: OutsideGuard) => { local_id } (& LocalsForNode :: One (_) , ForGuard :: RefWithinGuard) => { bug ! ("anything with one local should never be within a guard.") } } } }
    };
}

impl_15!()