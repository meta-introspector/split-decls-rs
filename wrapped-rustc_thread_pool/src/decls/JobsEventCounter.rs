macro_rules! JobsEventCounter {
    () => {
        # [doc = " A value read from the **Jobs Event Counter**."] # [doc = " See the [`README.md`](README.md) for more"] # [doc = " coverage of how the jobs event counter works."] # [derive (Copy , Clone , Debug , PartialEq , PartialOrd)] pub (super) struct JobsEventCounter (usize) ;
    };
}

JobsEventCounter!();