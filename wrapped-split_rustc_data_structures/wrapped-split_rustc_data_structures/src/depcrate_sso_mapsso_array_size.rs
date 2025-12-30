// Generated macro for SSO_ARRAY_SIZE (const)
macro_rules! Depcrate_sso_mapSSO_ARRAY_SIZE {
() => {
// Module: crate::sso::map
// Provides: {"SSO_ARRAY_SIZE"}
// Dependencies: {}
# [doc = " For pointer-sized arguments arrays"] # [doc = " are faster than set/map for up to 64"] # [doc = " arguments."] # [doc = ""] # [doc = " On the other hand such a big array"] # [doc = " hurts cache performance, makes passing"] # [doc = " sso structures around very expensive."] # [doc = ""] # [doc = " Biggest performance benefit is gained"] # [doc = " for reasonably small arrays that stay"] # [doc = " small in vast majority of cases."] # [doc = ""] # [doc = " '8' is chosen as a sane default, to be"] # [doc = " reevaluated later."] const SSO_ARRAY_SIZE : usize = 8 ;
};
}
