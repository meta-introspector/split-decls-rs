macro_rules! BinderLevel {
    () => {
        struct BinderLevel { # [doc = " The range of distances from the root of what's"] # [doc = " being printed, to the lifetimes in a binder."] # [doc = " Specifically, a `BrAnon` lifetime has depth"] # [doc = " `lifetime_depths.start + index`, going away from the"] # [doc = " the root and towards its use site, as the var index increases."] # [doc = " This is used to flatten rustc's pairing of `BrAnon`"] # [doc = " (intra-binder disambiguation) with a `DebruijnIndex`"] # [doc = " (binder addressing), to \"true\" de Bruijn indices,"] # [doc = " by subtracting the depth of a certain lifetime, from"] # [doc = " the innermost depth at its use site."] lifetime_depths : Range < u32 > , }
    };
}

BinderLevel!();