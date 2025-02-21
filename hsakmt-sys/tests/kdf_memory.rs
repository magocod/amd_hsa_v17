use hsakmt_sys::bindings::{
    _HsaMemFlags__bindgen_ty_1, hsaKmtAcquireSystemProperties, hsaKmtAllocMemory, hsaKmtFreeMemory,
    hsaKmtMapMemoryToGPU, hsaKmtOpenKFD, hsaKmtUnmapMemoryToGPU, HsaMemFlags, HsaSystemProperties,
    _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS, _HSA_CACHING_TYPE_HSA_CACHING_NONCACHED,
    _HSA_PAGE_SIZE_HSA_PAGE_SIZE_4KB,
};

// Basic test of hsaKmtMapMemoryToGPU and hsaKmtUnmapMemoryToGPU
#[test]
fn test_map_memory_to_gpu() {
    let mut p_db: *mut std::os::raw::c_void = std::ptr::null_mut();

    let gpu_node = 1;
    let page_size = 1 << 12;

    let mut m_memory_flags = HsaMemFlags {
        __bindgen_anon_1: _HsaMemFlags__bindgen_ty_1 { Value: 1 },
    };

    let mut m_system_properties = HsaSystemProperties {
        NumNodes: 0,
        PlatformOem: 0,
        PlatformId: 0,
        PlatformRev: 0,
    };

    unsafe {
        let ret = hsaKmtOpenKFD();
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        // In order to be correctly testing the KFD interfaces and ensure
        // that the KFD acknowledges relevant node parameters
        // for the rest of the tests and used for more specific topology tests,
        // call to GetSystemProperties for a system snapshot of the topology here
        let ret = hsaKmtAcquireSystemProperties(&mut m_system_properties);
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        // setting memory flags with default values , can be modified according to needs
        m_memory_flags.__bindgen_anon_1.ui32.set_NonPaged(0); // Paged
        m_memory_flags
            .__bindgen_anon_1
            .ui32
            .set_CachePolicy(_HSA_CACHING_TYPE_HSA_CACHING_NONCACHED); // Non cached
        m_memory_flags.__bindgen_anon_1.ui32.set_ReadOnly(0); // Read/Write
        m_memory_flags
            .__bindgen_anon_1
            .ui32
            .set_PageSize(_HSA_PAGE_SIZE_HSA_PAGE_SIZE_4KB); // 4KB page
        m_memory_flags.__bindgen_anon_1.ui32.set_HostAccess(1); // Host accessible
        m_memory_flags.__bindgen_anon_1.ui32.set_NoSubstitute(0); // Fall back to node 0 if needed
        m_memory_flags.__bindgen_anon_1.ui32.set_GDSMemory(0);
        m_memory_flags.__bindgen_anon_1.ui32.set_Scratch(0);

        let ret = hsaKmtAllocMemory(gpu_node, page_size, m_memory_flags, &mut p_db);
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        // verify that pDb is not null before it's being used
        assert_ne!(p_db, std::ptr::null_mut());

        let ret = hsaKmtMapMemoryToGPU(p_db, page_size, std::ptr::null_mut());
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        let ret = hsaKmtUnmapMemoryToGPU(p_db);
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        // Release the buffers
        let ret = hsaKmtFreeMemory(p_db, page_size);
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);
    }
}
