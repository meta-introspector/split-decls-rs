macro_rules! deps {
    () => {
        KMer!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " A re-usable allocation containing large allocations"] # [doc = " that are used multiple times during dictionary construction (once per epoch)"] pub struct Context { # [doc = " Keeps track of the number of occurances of a particular k-mer within an epoch."] # [doc = ""] # [doc = " Reset for each epoch."] pub frequencies : HashMap < KMer , usize > , }
    };
}

Context!()