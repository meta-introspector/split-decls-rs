macro_rules! InterfaceIndexOrAddress {
    () => {
        # [doc = " A local interface specified by its index or an address assigned to it."] # [doc = ""] # [doc = " `Index(0)` and `Address(Ipv4Addr::UNSPECIFIED)` are equivalent and indicate"] # [doc = " that an appropriate interface should be selected by the system."] # [cfg (not (any (target_os = "haiku" , target_os = "illumos" , target_os = "netbsd" , target_os = "redox" , target_os = "solaris" ,)))] # [derive (Debug , Copy , Clone)] pub enum InterfaceIndexOrAddress { # [doc = " An interface index."] Index (u32) , # [doc = " An address assigned to an interface."] Address (Ipv4Addr) , }
    };
}

InterfaceIndexOrAddress!()