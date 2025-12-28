macro_rules! deps {
    () => {
        CompatCapability!();
        Capability!();
    };
}

macro_rules! impl_1297 {
    () => {
        deps!();
        # [allow (deprecated)] impl CompatCapability for Capability { fn as_capability_set (self , _ : private :: Token) -> CapabilitySet { match self { Self :: ChangeOwnership => CapabilitySet :: CHOWN , Self :: DACOverride => CapabilitySet :: DAC_OVERRIDE , Self :: DACReadSearch => CapabilitySet :: DAC_READ_SEARCH , Self :: FileOwner => CapabilitySet :: FOWNER , Self :: FileSetID => CapabilitySet :: FSETID , Self :: Kill => CapabilitySet :: KILL , Self :: SetGroupID => CapabilitySet :: SETGID , Self :: SetUserID => CapabilitySet :: SETUID , Self :: SetPermittedCapabilities => CapabilitySet :: SETPCAP , Self :: LinuxImmutable => CapabilitySet :: LINUX_IMMUTABLE , Self :: NetBindService => CapabilitySet :: NET_BIND_SERVICE , Self :: NetBroadcast => CapabilitySet :: NET_BROADCAST , Self :: NetAdmin => CapabilitySet :: NET_ADMIN , Self :: NetRaw => CapabilitySet :: NET_RAW , Self :: IPCLock => CapabilitySet :: IPC_LOCK , Self :: IPCOwner => CapabilitySet :: IPC_OWNER , Self :: SystemModule => CapabilitySet :: SYS_MODULE , Self :: SystemRawIO => CapabilitySet :: SYS_RAWIO , Self :: SystemChangeRoot => CapabilitySet :: SYS_CHROOT , Self :: SystemProcessTrace => CapabilitySet :: SYS_PTRACE , Self :: SystemProcessAccounting => CapabilitySet :: SYS_PACCT , Self :: SystemAdmin => CapabilitySet :: SYS_ADMIN , Self :: SystemBoot => CapabilitySet :: SYS_BOOT , Self :: SystemNice => CapabilitySet :: SYS_NICE , Self :: SystemResource => CapabilitySet :: SYS_RESOURCE , Self :: SystemTime => CapabilitySet :: SYS_TIME , Self :: SystemTTYConfig => CapabilitySet :: SYS_TTY_CONFIG , Self :: MakeNode => CapabilitySet :: MKNOD , Self :: Lease => CapabilitySet :: LEASE , Self :: AuditWrite => CapabilitySet :: AUDIT_WRITE , Self :: AuditControl => CapabilitySet :: AUDIT_CONTROL , Self :: SetFileCapabilities => CapabilitySet :: SETFCAP , Self :: MACOverride => CapabilitySet :: MAC_OVERRIDE , Self :: MACAdmin => CapabilitySet :: MAC_ADMIN , Self :: SystemLog => CapabilitySet :: SYSLOG , Self :: WakeAlarm => CapabilitySet :: WAKE_ALARM , Self :: BlockSuspend => CapabilitySet :: BLOCK_SUSPEND , Self :: AuditRead => CapabilitySet :: AUDIT_READ , Self :: PerformanceMonitoring => CapabilitySet :: PERFMON , Self :: BerkeleyPacketFilters => CapabilitySet :: BPF , Self :: CheckpointRestore => CapabilitySet :: CHECKPOINT_RESTORE , } } }
    };
}

impl_1297!()