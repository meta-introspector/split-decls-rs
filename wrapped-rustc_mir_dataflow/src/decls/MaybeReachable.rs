macro_rules! MaybeReachable {
    () => {
        # [doc = " Extend a lattice with a bottom value to represent an unreachable execution."] # [doc = ""] # [doc = " The only useful action on an unreachable state is joining it with a reachable one to make it"] # [doc = " reachable. All other actions, gen/kill for instance, are no-ops."] # [derive (PartialEq , Eq , Debug)] pub enum MaybeReachable < T > { Unreachable , Reachable (T) , }
    };
}

MaybeReachable!();