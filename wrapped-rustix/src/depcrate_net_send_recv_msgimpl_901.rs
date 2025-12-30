// Generated macro for impl_901 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_901 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_901"}
// Dependencies: {}
impl < 'buf > Iterator for AncillaryDrain < 'buf > { type Item = RecvAncillaryMessage < 'buf > ; fn next (& mut self) -> Option < Self :: Item > { self . messages . find_map (| ev | Self :: advance (& mut self . read_and_length , ev)) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , max) = self . messages . size_hint () ; (0 , max) } fn fold < B , F > (mut self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . messages . filter_map (| ev | Self :: advance (& mut self . read_and_length , ev)) . fold (init , f) } fn count (mut self) -> usize { self . messages . filter_map (| ev | Self :: advance (& mut self . read_and_length , ev)) . count () } fn last (mut self) -> Option < Self :: Item > where Self : Sized , { self . messages . filter_map (| ev | Self :: advance (& mut self . read_and_length , ev)) . last () } fn collect < B : FromIterator < Self :: Item > > (mut self) -> B where Self : Sized , { self . messages . filter_map (| ev | Self :: advance (& mut self . read_and_length , ev)) . collect () } }
};
}
