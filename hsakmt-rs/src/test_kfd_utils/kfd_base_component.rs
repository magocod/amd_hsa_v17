#![allow(non_snake_case, dead_code)]

use crate::globals::HsakmtGlobals;
use crate::hsakmttypes::{HsaMemFlags, HsaSystemProperties, _HsaMemFlags__bindgen_ty_1, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS};

pub struct KFDBaseComponentTest {
    pub hsakmt: HsakmtGlobals,
    pub m_SystemProperties: HsaSystemProperties,
    // pub m_MemoryFlags: HsaMemFlags,
}

impl KFDBaseComponentTest {
    pub fn new() -> Self {
        Self {
            hsakmt: HsakmtGlobals::new(),
            m_SystemProperties: HsaSystemProperties {
                NumNodes: 0,
                PlatformOem: 0,
                PlatformId: 0,
                PlatformRev: 0,
            },
        }
    }

    pub unsafe fn set_up(&mut self) {
        let ret = self.hsakmt.hsaKmtOpenKFD();
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        // In order to be correctly testing the KFD interfaces and ensure
        // that the KFD acknowledges relevant node parameters
        // for the rest of the tests and used for more specific topology tests,
        // call to GetSystemProperties for a system snapshot of the topology here
        let ret = self
            .hsakmt
            .hsaKmtAcquireSystemProperties(&mut self.m_SystemProperties);
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        // setting memory flags with default values , can be modified according to needs
        // self.m_MemoryFlags.st.ui32.NonPaged = 0; // Paged
        // self.m_MemoryFlags.st.ui32.CachePolicy = HSA_CACHING_NONCACHED as u32; // Non cached
        // self.m_MemoryFlags.st.ui32.ReadOnly = 0; // Read/Write
        // self.m_MemoryFlags.st.ui32.PageSize = HSA_PAGE_SIZE_4KB as u32; // 4KB page
        // self.m_MemoryFlags.st.ui32.HostAccess = 1; // Host accessible
        // self.m_MemoryFlags.st.ui32.NoSubstitute = 0; // Fall back to node 0 if needed
        // self.m_MemoryFlags.st.ui32.GDSMemory = 0;
        // self.m_MemoryFlags.st.ui32.Scratch = 0;
    }
}

impl Default for KFDBaseComponentTest {
    fn default() -> Self {
        Self::new()
    }
}
