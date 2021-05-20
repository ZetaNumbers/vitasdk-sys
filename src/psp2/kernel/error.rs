/* automatically generated by rust-bindgen 0.58.1 */

pub mod SceKernelErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_OK: Type = 0;
    pub const SCE_KERNEL_ERROR_ERROR: Type = 2147614721;
    pub const SCE_KERNEL_ERROR_NOT_IMPLEMENTED: Type = 2147614722;
    pub const SCE_KERNEL_ERROR_NOSYS: Type = 2147614723;
    pub const SCE_KERNEL_ERROR_UNSUP: Type = 2147614724;
    pub const SCE_KERNEL_ERROR_INVALID_ARGUMENT: Type = 2147614725;
    pub const SCE_KERNEL_ERROR_ILLEGAL_ADDR: Type = 2147614726;
    pub const SCE_KERNEL_ERROR_ILLEGAL_ALIGNMENT: Type = 2147614727;
    pub const SCE_KERNEL_ERROR_ILLEGAL_PERMISSION: Type = 2147614728;
    pub const SCE_KERNEL_ERROR_INVALID_ARGUMENT_SIZE: Type = 2147614729;
    pub const SCE_KERNEL_ERROR_INVALID_FLAGS: Type = 2147614730;
    pub const SCE_KERNEL_ERROR_ILLEGAL_SIZE: Type = 2147614731;
    pub const SCE_KERNEL_ERROR_ILLEGAL_TYPE: Type = 2147614732;
    pub const SCE_KERNEL_ERROR_ILLEGAL_PATTERN: Type = 2147614733;
    pub const SCE_KERNEL_ERROR_ILLEGAL_ATTR: Type = 2147614734;
    pub const SCE_KERNEL_ERROR_ILLEGAL_COUNT: Type = 2147614735;
    pub const SCE_KERNEL_ERROR_ILLEGAL_MODE: Type = 2147614736;
    pub const SCE_KERNEL_ERROR_ILLEGAL_OPEN_LIMIT: Type = 2147614737;
    pub const SCE_KERNEL_ERROR_ONLY_DEVELOPMENT_MODE: Type = 2147614738;
    pub const SCE_KERNEL_ERROR_DEBUG_ERROR: Type = 2147618816;
    pub const SCE_KERNEL_ERROR_ILLEGAL_DIPSW_NUMBER: Type = 2147618817;
    pub const SCE_KERNEL_ERROR_PA_ERROR: Type = 2147619072;
    pub const SCE_KERNEL_ERROR_PA_NOT_AVAILABLE: Type = 2147619073;
    pub const SCE_KERNEL_ERROR_PA_INVALID_KEY: Type = 2147619074;
    pub const SCE_KERNEL_ERROR_PA_KEY_IS_NOT_SHARED: Type = 2147619075;
    pub const SCE_KERNEL_ERROR_PA_INVALID_SIGNATURE: Type = 2147619076;
    pub const SCE_KERNEL_ERROR_CPU_ERROR: Type = 2147622912;
    pub const SCE_KERNEL_ERROR_MMU_ILLEGAL_L1_TYPE: Type = 2147622913;
    pub const SCE_KERNEL_ERROR_MMU_L2_INDEX_OVERFLOW: Type = 2147622914;
    pub const SCE_KERNEL_ERROR_MMU_L2_SIZE_OVERFLOW: Type = 2147622915;
    pub const SCE_KERNEL_ERROR_INVALID_CPU_AFFINITY: Type = 2147622916;
    pub const SCE_KERNEL_ERROR_INVALID_MEMORY_ACCESS: Type = 2147622917;
    pub const SCE_KERNEL_ERROR_INVALID_MEMORY_ACCESS_PERMISSION: Type = 2147622918;
    pub const SCE_KERNEL_ERROR_VA2PA_FAULT: Type = 2147622919;
    pub const SCE_KERNEL_ERROR_VA2PA_MAPPED: Type = 2147622920;
    pub const SCE_KERNEL_ERROR_VALIDATION_CHECK_FAILED: Type = 2147622921;
    pub const SCE_KERNEL_ERROR_SYSMEM_ERROR: Type = 2147631104;
    pub const SCE_KERNEL_ERROR_INVALID_PROCESS_CONTEXT: Type = 2147631105;
    pub const SCE_KERNEL_ERROR_UID_NAME_TOO_LONG: Type = 2147631106;
    pub const SCE_KERNEL_ERROR_VARANGE_IS_NOT_PHYSICAL_CONTINUOUS: Type = 2147631107;
    pub const SCE_KERNEL_ERROR_PHYADDR_ERROR: Type = 2147631360;
    pub const SCE_KERNEL_ERROR_NO_PHYADDR: Type = 2147631361;
    pub const SCE_KERNEL_ERROR_PHYADDR_USED: Type = 2147631362;
    pub const SCE_KERNEL_ERROR_PHYADDR_NOT_USED: Type = 2147631363;
    pub const SCE_KERNEL_ERROR_NO_IOADDR: Type = 2147631364;
    pub const SCE_KERNEL_ERROR_PHYMEM_ERROR: Type = 2147631872;
    pub const SCE_KERNEL_ERROR_ILLEGAL_PHYPAGE_STATUS: Type = 2147631873;
    pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE: Type = 2147631874;
    pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE_UNIT: Type = 2147631875;
    pub const SCE_KERNEL_ERROR_PHYMEMPART_NOT_EMPTY: Type = 2147631876;
    pub const SCE_KERNEL_ERROR_NO_PHYMEMPART_LPDDR2: Type = 2147631877;
    pub const SCE_KERNEL_ERROR_NO_PHYMEMPART_CDRAM: Type = 2147631878;
    pub const SCE_KERNEL_ERROR_PHYMEMPART_OUT_OF_INDEX: Type = 2147631879;
    pub const SCE_KERNEL_ERROR_CANNOT_GROW_PHYMEMPART: Type = 2147631880;
    pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE_CDRAM: Type = 2147631881;
    pub const SCE_KERNEL_ERROR_INVALID_SUBBUDGET_ID: Type = 2147631882;
    pub const SCE_KERNEL_ERROR_FIXEDHEAP_ERROR: Type = 2147632128;
    pub const SCE_KERNEL_ERROR_FIXEDHEAP_ILLEGAL_SIZE: Type = 2147632129;
    pub const SCE_KERNEL_ERROR_FIXEDHEAP_ILLEGAL_INDEX: Type = 2147632130;
    pub const SCE_KERNEL_ERROR_FIXEDHEAP_INDEX_OVERFLOW: Type = 2147632131;
    pub const SCE_KERNEL_ERROR_FIXEDHEAP_NO_CHUNK: Type = 2147632132;
    pub const SCE_KERNEL_ERROR_UID_ERROR: Type = 2147632384;
    pub const SCE_KERNEL_ERROR_INVALID_UID: Type = 2147632385;
    pub const SCE_KERNEL_ERROR_SYSMEM_UID_INVALID_ARGUMENT: Type = 2147632386;
    pub const SCE_KERNEL_ERROR_SYSMEM_INVALID_UID_RANGE: Type = 2147632387;
    pub const SCE_KERNEL_ERROR_SYSMEM_NO_VALID_UID: Type = 2147632388;
    pub const SCE_KERNEL_ERROR_SYSMEM_CANNOT_ALLOCATE_UIDENTRY: Type = 2147632389;
    pub const SCE_KERNEL_ERROR_NOT_PROCESS_UID: Type = 2147632390;
    pub const SCE_KERNEL_ERROR_NOT_KERNEL_UID: Type = 2147632391;
    pub const SCE_KERNEL_ERROR_INVALID_UID_CLASS: Type = 2147632392;
    pub const SCE_KERNEL_ERROR_INVALID_UID_SUBCLASS: Type = 2147632393;
    pub const SCE_KERNEL_ERROR_UID_CANNOT_FIND_BY_NAME: Type = 2147632394;
    pub const SCE_KERNEL_ERROR_UID_NOT_VISIBLE: Type = 2147632395;
    pub const SCE_KERNEL_ERROR_UID_MAX_OPEN: Type = 2147632396;
    pub const SCE_KERNEL_ERROR_UID_RL_OVERFLOW: Type = 2147632397;
    pub const SCE_KERNEL_ERROR_VIRPAGE_ERROR: Type = 2147632640;
    pub const SCE_KERNEL_ERROR_ILLEGAL_VIRPAGE_TYPE: Type = 2147632641;
    pub const SCE_KERNEL_ERROR_BLOCK_ERROR: Type = 2147632896;
    pub const SCE_KERNEL_ERROR_ILLEGAL_BLOCK_ID: Type = 2147632897;
    pub const SCE_KERNEL_ERROR_ILLEGAL_BLOCK_TYPE: Type = 2147632898;
    pub const SCE_KERNEL_ERROR_BLOCK_IN_USE: Type = 2147632899;
    pub const SCE_KERNEL_ERROR_PARTITION_ERROR: Type = 2147633152;
    pub const SCE_KERNEL_ERROR_ILLEGAL_PARTITION_ID: Type = 2147633153;
    pub const SCE_KERNEL_ERROR_ILLEGAL_PARTITION_INDEX: Type = 2147633154;
    pub const SCE_KERNEL_ERROR_NO_L2PAGETABLE: Type = 2147633155;
    pub const SCE_KERNEL_ERROR_HEAPLIB_ERROR: Type = 2147633408;
    pub const SCE_KERNEL_ERROR_ILLEGAL_HEAP_ID: Type = 2147633409;
    pub const SCE_KERNEL_ERROR_OUT_OF_RANG: Type = 2147633410;
    pub const SCE_KERNEL_ERROR_HEAPLIB_NOMEM: Type = 2147633411;
    pub const SCE_KERNEL_ERROR_HEAPLIB_VERIFY_ERROR: Type = 2147633412;
    pub const SCE_KERNEL_ERROR_SYSMEM_ADDRESS_SPACE_ERROR: Type = 2147633664;
    pub const SCE_KERNEL_ERROR_INVALID_ADDRESS_SPACE_ID: Type = 2147633665;
    pub const SCE_KERNEL_ERROR_INVALID_PARTITION_INDEX: Type = 2147633666;
    pub const SCE_KERNEL_ERROR_ADDRESS_SPACE_CANNOT_FIND_PARTITION_BY_ADDR: Type = 2147633667;
    pub const SCE_KERNEL_ERROR_SYSMEM_MEMBLOCK_ERROR: Type = 2147633920;
    pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_TYPE: Type = 2147633921;
    pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_REMAP_TYPE: Type = 2147633922;
    pub const SCE_KERNEL_ERROR_NOT_PHY_CONT_MEMBLOCK: Type = 2147633923;
    pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_CODE: Type = 2147633924;
    pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_SIZE: Type = 2147633925;
    pub const SCE_KERNEL_ERROR_ILLEGAL_USERMAP_SIZE: Type = 2147633926;
    pub const SCE_KERNEL_ERROR_MEMBLOCK_TYPE_FOR_KERNEL_PROCESS: Type = 2147633927;
    pub const SCE_KERNEL_ERROR_PROCESS_CANNOT_REMAP_MEMBLOCK: Type = 2147633928;
    pub const SCE_KERNEL_ERROR_MEMBLOCK_RANGE_ERROR: Type = 2147633929;
    pub const SCE_KERNEL_ERROR_MEMBLOCK_TYPE_FOR_UPDATER_OR_SAFEMODE: Type = 2147633930;
    pub const SCE_KERNEL_ERROR_MEMBLOCK_OVERFLOW: Type = 2147633931;
    pub const SCE_KERNEL_ERROR_SYSMEM_PHYMEMLOW_ERROR: Type = 2147634176;
    pub const SCE_KERNEL_ERROR_CANNOT_ALLOC_PHYMEMLOW: Type = 2147634177;
    pub const SCE_KERNEL_ERROR_UNKNOWN_PHYMEMLOW_TYPE: Type = 2147634178;
    pub const SCE_KERNEL_ERROR_SYSMEM_BITHEAP_ERROR: Type = 2147634432;
    pub const SCE_KERNEL_ERROR_CANNOT_ALLOC_BITHEAP: Type = 2147634433;
    pub const SCE_KERNEL_ERROR_SYSMEM_NAMEHEAP_ERROR: Type = 2147634688;
    pub const SCE_KERNEL_ERROR_NO_SUCH_NAME: Type = 2147634689;
    pub const SCE_KERNEL_ERROR_DUPLICATE_NAME: Type = 2147634690;
    pub const SCE_KERNEL_ERROR_LOADCORE_ERROR: Type = 2147635200;
    pub const SCE_KERNEL_ERROR_ILLEGAL_ELF_HEADER: Type = 2147635201;
    pub const SCE_KERNEL_ERROR_ILLEGAL_SELF_HEADER: Type = 2147635202;
    pub const SCE_KERNEL_ERROR_EXCPMGR_ERROR: Type = 2147643392;
    pub const SCE_KERNEL_ERROR_ILLEGAL_EXCPCODE: Type = 2147643393;
    pub const SCE_KERNEL_ERROR_ILLEGAL_EXCPHANDLER: Type = 2147643394;
    pub const SCE_KERNEL_ERROR_NOTFOUND_EXCPHANDLER: Type = 2147643395;
    pub const SCE_KERNEL_ERROR_CANNOT_RELEASE_EXCPHANDLER: Type = 2147643396;
    pub const SCE_KERNEL_ERROR_INTRMGR_ERROR: Type = 2147643648;
    pub const SCE_KERNEL_ERROR_ILLEGAL_CONTEXT: Type = 2147643649;
    pub const SCE_KERNEL_ERROR_ILLEGAL_INTRCODE: Type = 2147643650;
    pub const SCE_KERNEL_ERROR_ILLEGAL_INTRPARAM: Type = 2147643651;
    pub const SCE_KERNEL_ERROR_ILLEGAL_INTRPRIORITY: Type = 2147643652;
    pub const SCE_KERNEL_ERROR_ILLEGAL_TARGET_CPU: Type = 2147643653;
    pub const SCE_KERNEL_ERROR_ILLEGAL_INTRFILTER: Type = 2147643654;
    pub const SCE_KERNEL_ERROR_ILLEGAL_INTRTYPE: Type = 2147643655;
    pub const SCE_KERNEL_ERROR_ILLEGAL_HANDLER: Type = 2147643656;
    pub const SCE_KERNEL_ERROR_FOUND_HANDLER: Type = 2147643657;
    pub const SCE_KERNEL_ERROR_NOTFOUND_HANDLER: Type = 2147643658;
    pub const SCE_KERNEL_ERROR_NO_MEMORY: Type = 2147643659;
    pub const SCE_KERNEL_ERROR_DMACMGR_ERROR: Type = 2147643904;
    pub const SCE_KERNEL_ERROR_ALREADY_QUEUED: Type = 2147643905;
    pub const SCE_KERNEL_ERROR_NOT_QUEUED: Type = 2147643906;
    pub const SCE_KERNEL_ERROR_NOT_SETUP: Type = 2147643907;
    pub const SCE_KERNEL_ERROR_ON_TRANSFERRING: Type = 2147643908;
    pub const SCE_KERNEL_ERROR_NOT_INITIALIZED: Type = 2147643909;
    pub const SCE_KERNEL_ERROR_TRANSFERRED: Type = 2147643910;
    pub const SCE_KERNEL_ERROR_NOT_UNDER_CONTROL: Type = 2147643911;
    pub const SCE_KERNEL_ERROR_CANCELING: Type = 2147643912;
    pub const SCE_KERNEL_ERROR_SYSTIMER_ERROR: Type = 2147644160;
    pub const SCE_KERNEL_ERROR_NO_FREE_TIMER: Type = 2147644161;
    pub const SCE_KERNEL_ERROR_TIMER_NOT_ALLOCATED: Type = 2147644162;
    pub const SCE_KERNEL_ERROR_TIMER_COUNTING: Type = 2147644163;
    pub const SCE_KERNEL_ERROR_TIMER_STOPPED: Type = 2147644164;
    pub const SCE_KERNEL_ERROR_THREADMGR_ERROR: Type = 2147647488;
    pub const SCE_KERNEL_ERROR_UNKNOWN_UID: Type = 2147647489;
    pub const SCE_KERNEL_ERROR_DIFFERENT_UID_CLASS: Type = 2147647490;
    pub const SCE_KERNEL_ERROR_ALREADY_REGISTERED: Type = 2147647491;
    pub const SCE_KERNEL_ERROR_CAN_NOT_WAIT: Type = 2147647492;
    pub const SCE_KERNEL_ERROR_WAIT_TIMEOUT: Type = 2147647493;
    pub const SCE_KERNEL_ERROR_WAIT_DELETE: Type = 2147647494;
    pub const SCE_KERNEL_ERROR_WAIT_CANCEL: Type = 2147647495;
    pub const SCE_KERNEL_ERROR_THREAD_ERROR: Type = 2147647520;
    pub const SCE_KERNEL_ERROR_UNKNOWN_THREAD_ID: Type = 2147647521;
    pub const SCE_KERNEL_ERROR_ILLEGAL_THREAD_ID: Type = 2147647522;
    pub const SCE_KERNEL_ERROR_ILLEGAL_PRIORITY: Type = 2147647523;
    pub const SCE_KERNEL_ERROR_ILLEGAL_STACK_SIZE: Type = 2147647524;
    pub const SCE_KERNEL_ERROR_ILLEGAL_CPU_AFFINITY_MASK: Type = 2147647525;
    pub const SCE_KERNEL_ERROR_ILLEGAL_THREAD_PARAM_COMBINATION: Type = 2147647526;
    pub const SCE_KERNEL_ERROR_DORMANT: Type = 2147647527;
    pub const SCE_KERNEL_ERROR_NOT_DORMANT: Type = 2147647528;
    pub const SCE_KERNEL_ERROR_RUNNING: Type = 2147647529;
    pub const SCE_KERNEL_ERROR_DELETED: Type = 2147647530;
    pub const SCE_KERNEL_ERROR_CAN_NOT_SUSPEND: Type = 2147647531;
    pub const SCE_KERNEL_ERROR_THREAD_STOPPED: Type = 2147647532;
    pub const SCE_KERNEL_ERROR_THREAD_SUSPENDED: Type = 2147647533;
    pub const SCE_KERNEL_ERROR_NOT_SUSPENDED: Type = 2147647534;
    pub const SCE_KERNEL_ERROR_ALREADY_DEBUG_SUSPENDED: Type = 2147647535;
    pub const SCE_KERNEL_ERROR_NOT_DEBUG_SUSPENDED: Type = 2147647536;
    pub const SCE_KERNEL_ERROR_CAN_NOT_USE_VFP: Type = 2147647537;
    pub const SCE_KERNEL_ERROR_THREAD_EVENT_ERROR: Type = 2147647584;
    pub const SCE_KERNEL_ERROR_UNKNOWN_THREAD_EVENT_ID: Type = 2147647585;
    pub const SCE_KERNEL_ERROR_KERNEL_TLS_ERROR: Type = 2147647616;
    pub const SCE_KERNEL_ERROR_KERNEL_TLS_FULL: Type = 2147647617;
    pub const SCE_KERNEL_ERROR_ILLEGAL_KERNEL_TLS_INDEX: Type = 2147647618;
    pub const SCE_KERNEL_ERROR_KERNEL_TLS_BUSY: Type = 2147647619;
    pub const SCE_KERNEL_ERROR_CALLBACK_ERROR: Type = 2147647648;
    pub const SCE_KERNEL_ERROR_UNKNOWN_CALLBACK_ID: Type = 2147647649;
    pub const SCE_KERNEL_ERROR_NOTIFY_CALLBACK: Type = 2147647650;
    pub const SCE_KERNEL_ERROR_CALLBACK_NOT_REGISTERED: Type = 2147647651;
    pub const SCE_KERNEL_ERROR_ALARM_ERROR: Type = 2147647680;
    pub const SCE_KERNEL_ERROR_UNKNOWN_ALARM_ID: Type = 2147647681;
    pub const SCE_KERNEL_ERROR_ALARM_CAN_NOT_CANCEL: Type = 2147647682;
    pub const SCE_KERNEL_ERROR_EVF_ERROR: Type = 2147647712;
    pub const SCE_KERNEL_ERROR_UNKNOWN_EVF_ID: Type = 2147647713;
    pub const SCE_KERNEL_ERROR_EVF_MULTI: Type = 2147647714;
    pub const SCE_KERNEL_ERROR_EVF_COND: Type = 2147647715;
    pub const SCE_KERNEL_ERROR_SEMA_ERROR: Type = 2147647744;
    pub const SCE_KERNEL_ERROR_UNKNOWN_SEMA_ID: Type = 2147647745;
    pub const SCE_KERNEL_ERROR_SEMA_ZERO: Type = 2147647746;
    pub const SCE_KERNEL_ERROR_SEMA_OVF: Type = 2147647747;
    pub const SCE_KERNEL_ERROR_SIGNAL_ERROR: Type = 2147647776;
    pub const SCE_KERNEL_ERROR_ALREADY_SENT: Type = 2147647777;
    pub const SCE_KERNEL_ERROR_MUTEX_ERROR: Type = 2147647808;
    pub const SCE_KERNEL_ERROR_UNKNOWN_MUTEX_ID: Type = 2147647809;
    pub const SCE_KERNEL_ERROR_MUTEX_RECURSIVE: Type = 2147647810;
    pub const SCE_KERNEL_ERROR_MUTEX_LOCK_OVF: Type = 2147647811;
    pub const SCE_KERNEL_ERROR_MUTEX_UNLOCK_UDF: Type = 2147647812;
    pub const SCE_KERNEL_ERROR_MUTEX_FAILED_TO_OWN: Type = 2147647813;
    pub const SCE_KERNEL_ERROR_MUTEX_NOT_OWNED: Type = 2147647814;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_ERROR: Type = 2147647840;
    pub const SCE_KERNEL_ERROR_UNKNOWN_FAST_MUTEX_ID: Type = 2147647841;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_RECURSIVE: Type = 2147647842;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_LOCK_OVF: Type = 2147647843;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_FAILED_TO_OWN: Type = 2147647844;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_NOT_OWNED: Type = 2147647845;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_OWNED: Type = 2147647846;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_ALREADY_INITIALIZED: Type = 2147647847;
    pub const SCE_KERNEL_ERROR_FAST_MUTEX_NOT_INITIALIZED: Type = 2147647848;
    pub const SCE_KERNEL_ERROR_LW_MUTEX_ERROR: Type = 2147647872;
    pub const SCE_KERNEL_ERROR_UNKNOWN_LW_MUTEX_ID: Type = 2147647873;
    pub const SCE_KERNEL_ERROR_LW_MUTEX_RECURSIVE: Type = 2147647874;
    pub const SCE_KERNEL_ERROR_LW_MUTEX_LOCK_OVF: Type = 2147647875;
    pub const SCE_KERNEL_ERROR_LW_MUTEX_UNLOCK_UDF: Type = 2147647876;
    pub const SCE_KERNEL_ERROR_LW_MUTEX_FAILED_TO_OWN: Type = 2147647877;
    pub const SCE_KERNEL_ERROR_LW_MUTEX_NOT_OWNED: Type = 2147647878;
    pub const SCE_KERNEL_ERROR_COND_ERROR: Type = 2147647904;
    pub const SCE_KERNEL_ERROR_UNKNOWN_COND_ID: Type = 2147647905;
    pub const SCE_KERNEL_ERROR_WAIT_DELETE_MUTEX: Type = 2147647906;
    pub const SCE_KERNEL_ERROR_WAIT_CANCEL_MUTEX: Type = 2147647907;
    pub const SCE_KERNEL_ERROR_WAIT_DELETE_COND: Type = 2147647908;
    pub const SCE_KERNEL_ERROR_WAIT_CANCEL_COND: Type = 2147647909;
    pub const SCE_KERNEL_ERROR_LW_COND_ERROR: Type = 2147647936;
    pub const SCE_KERNEL_ERROR_UNKNOWN_LW_COND_ID: Type = 2147647937;
    pub const SCE_KERNEL_ERROR_WAIT_DELETE_LW_MUTEX: Type = 2147647938;
    pub const SCE_KERNEL_ERROR_WAIT_DELETE_LW_COND: Type = 2147647939;
    pub const SCE_KERNEL_ERROR_RW_LOCK_ERROR: Type = 2147647968;
    pub const SCE_KERNEL_ERROR_UNKNOWN_RW_LOCK_ID: Type = 2147647969;
    pub const SCE_KERNEL_ERROR_RW_LOCK_RECURSIVE: Type = 2147647970;
    pub const SCE_KERNEL_ERROR_RW_LOCK_LOCK_OVF: Type = 2147647971;
    pub const SCE_KERNEL_ERROR_RW_LOCK_NOT_OWNED: Type = 2147647972;
    pub const SCE_KERNEL_ERROR_RW_LOCK_UNLOCK_UDF: Type = 2147647973;
    pub const SCE_KERNEL_ERROR_RW_LOCK_FAILED_TO_LOCK: Type = 2147647974;
    pub const SCE_KERNEL_ERROR_RW_LOCK_FAILED_TO_UNLOCK: Type = 2147647975;
    pub const SCE_KERNEL_ERROR_EVENT_ERROR: Type = 2147648000;
    pub const SCE_KERNEL_ERROR_UNKNOWN_EVENT_ID: Type = 2147648001;
    pub const SCE_KERNEL_ERROR_EVENT_COND: Type = 2147648002;
    pub const SCE_KERNEL_ERROR_MSG_PIPE_ERROR: Type = 2147648032;
    pub const SCE_KERNEL_ERROR_UNKNOWN_MSG_PIPE_ID: Type = 2147648033;
    pub const SCE_KERNEL_ERROR_MSG_PIPE_FULL: Type = 2147648034;
    pub const SCE_KERNEL_ERROR_MSG_PIPE_EMPTY: Type = 2147648035;
    pub const SCE_KERNEL_ERROR_MSG_PIPE_DELETED: Type = 2147648036;
    pub const SCE_KERNEL_ERROR_TIMER_ERROR: Type = 2147648064;
    pub const SCE_KERNEL_ERROR_UNKNOWN_TIMER_ID: Type = 2147648065;
    pub const SCE_KERNEL_ERROR_EVENT_NOT_SET: Type = 2147648066;
    pub const SCE_KERNEL_ERROR_SIMPLE_EVENT_ERROR: Type = 2147648096;
    pub const SCE_KERNEL_ERROR_UNKNOWN_SIMPLE_EVENT_ID: Type = 2147648097;
    pub const SCE_KERNEL_ERROR_PMON_ERROR: Type = 2147648128;
    pub const SCE_KERNEL_ERROR_PMON_NOT_THREAD_MODE: Type = 2147648129;
    pub const SCE_KERNEL_ERROR_PMON_NOT_CPU_MODE: Type = 2147648130;
    pub const SCE_KERNEL_ERROR_WORK_QUEUE: Type = 2147648256;
    pub const SCE_KERNEL_ERROR_UNKNOWN_WORK_QUEUE_ID: Type = 2147648257;
    pub const SCE_KERNEL_ERROR_UNKNOWN_WORK_TASK_ID: Type = 2147648258;
    pub const SCE_KERNEL_ERROR_PROCESSMGR_ERROR: Type = 2147651584;
    pub const SCE_KERNEL_ERROR_INVALID_PID: Type = 2147651585;
    pub const SCE_KERNEL_ERROR_INVALID_PROCESS_TYPE: Type = 2147651586;
    pub const SCE_KERNEL_ERROR_PLS_FULL: Type = 2147651587;
    pub const SCE_KERNEL_ERROR_INVALID_PROCESS_STATUS: Type = 2147651588;
    pub const SCE_KERNEL_ERROR_PROCESS_CALLBACK_NOTFOUND: Type = 2147651589;
    pub const SCE_KERNEL_ERROR_INVALID_BUDGET_ID: Type = 2147651590;
    pub const SCE_KERNEL_ERROR_INVALID_BUDGET_SIZE: Type = 2147651591;
    pub const SCE_KERNEL_ERROR_CP14_DISABLED: Type = 2147651592;
    pub const SCE_KERNEL_ERROR_EXCEEDED_MAX_PROCESSES: Type = 2147651593;
    pub const SCE_KERNEL_ERROR_PROCESS_REMAINING: Type = 2147651594;
    pub const SCE_KERNEL_ERROR_NO_PROCESS_DATA: Type = 2147651595;
    pub const SCE_KERNEL_ERROR_PROCESS_EVENT_INHIBITED: Type = 2147651596;
    pub const SCE_KERNEL_ERROR_IOFILEMGR_ERROR: Type = 2147655680;
    pub const SCE_KERNEL_ERROR_IO_NAME_TOO_LONG: Type = 2147655681;
    pub const SCE_KERNEL_ERROR_IO_REG_DEV: Type = 2147655682;
    pub const SCE_KERNEL_ERROR_IO_ALIAS_USED: Type = 2147655683;
    pub const SCE_KERNEL_ERROR_IO_DEL_DEV: Type = 2147655684;
    pub const SCE_KERNEL_ERROR_IO_WOULD_BLOCK: Type = 2147655685;
    pub const SCE_KERNEL_ERROR_MODULEMGR_START_FAILED: Type = 2147667968;
    pub const SCE_KERNEL_ERROR_MODULEMGR_STOP_FAIL: Type = 2147667969;
    pub const SCE_KERNEL_ERROR_MODULEMGR_IN_USE: Type = 2147667970;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NO_LIB: Type = 2147667971;
    pub const SCE_KERNEL_ERROR_MODULEMGR_SYSCALL_REG: Type = 2147667972;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_LIB: Type = 2147667973;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_STUB: Type = 2147667974;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_SELF: Type = 2147667975;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM: Type = 2147667976;
    pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_LIB: Type = 2147667977;
    pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_STUB: Type = 2147667978;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NO_FUNC_NID: Type = 2147667979;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NO_VAR_NID: Type = 2147667980;
    pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_TYPE: Type = 2147667981;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MOD_ENTRY: Type = 2147667982;
    pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_PROC_PARAM: Type = 2147667983;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MODOBJ: Type = 2147667984;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MOD: Type = 2147667985;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NO_PROCESS: Type = 2147667986;
    pub const SCE_KERNEL_ERROR_MODULEMGR_OLD_LIB: Type = 2147667987;
    pub const SCE_KERNEL_ERROR_MODULEMGR_STARTED: Type = 2147667988;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOT_STARTED: Type = 2147667989;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOT_STOPPED: Type = 2147667990;
    pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_PROCESS_UID: Type = 2147667991;
    pub const SCE_KERNEL_ERROR_MODULEMGR_CANNOT_EXPORT_LIB_TO_SHARED: Type = 2147667992;
    pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_REL_INFO: Type = 2147667993;
    pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_REF_INFO: Type = 2147667994;
    pub const SCE_KERNEL_ERROR_MODULEMGR_ELINK: Type = 2147667995;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOENT: Type = 2147667996;
    pub const SCE_KERNEL_ERROR_MODULEMGR_BUSY: Type = 2147667997;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NOEXEC: Type = 2147667998;
    pub const SCE_KERNEL_ERROR_MODULEMGR_NAMETOOLONG: Type = 2147667999;
    pub const SCE_KERNEL_ERROR_LIBRARYDB_NOENT: Type = 2147668096;
    pub const SCE_KERNEL_ERROR_LIBRARYDB_NO_LIB: Type = 2147668097;
    pub const SCE_KERNEL_ERROR_LIBRARYDB_NO_MOD: Type = 2147668098;
    pub const SCE_KERNEL_ERROR_PRELOAD_FAILED: Type = 2147668208;
    pub const SCE_KERNEL_ERROR_PRELOAD_LIBC_FAILED: Type = 2147668209;
    pub const SCE_KERNEL_ERROR_PRELOAD_FIOS2_FAILED: Type = 2147668210;
    pub const SCE_KERNEL_ERROR_AUTHFAIL: Type = 2147676160;
    pub const SCE_KERNEL_ERROR_NO_AUTH: Type = 2147676161;
}
