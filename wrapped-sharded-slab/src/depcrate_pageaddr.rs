// Generated macro for Addr (struct)
macro_rules! Depcrate_pageAddr {
() => {
// Module: crate::page
// Provides: {"Addr"}
// Dependencies: {}
# [doc = " A page address encodes the location of a slot within a shard (the page"] # [doc = " number and offset within that page) as a single linear value."] # [repr (transparent)] pub (crate) struct Addr < C : cfg :: Config = cfg :: DefaultConfig > { addr : usize , _cfg : PhantomData < fn (C) > , }
};
}
