#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use crate::fmm_types::svm_aperture_type::SVM_DEFAULT;
use crate::fmm_types::{gpu_mem_t, manageable_aperture_ops_t, HsakmtGlobalsArgs};
use crate::globals::HsakmtGlobals;
use crate::hsakmttypes::HsakmtStatus;
use crate::hsakmttypes::HsakmtStatus::{HSAKMT_STATUS_ERROR, HSAKMT_STATUS_SUCCESS};
use crate::kfd_ioctl::{
    kfd_ioctl_get_process_apertures_new_args, kfd_process_device_apertures,
    KFD_IOC_CACHE_POLICY_COHERENT, KFD_IOC_CACHE_POLICY_NONCOHERENT,
};
use crate::libhsakmt::hsakmt_ioctl;
use libc::{getenv, strcmp};
use std::ffi::CString;

impl HsakmtGlobals {
    // TODO complete fn get_vm_alignment
    pub fn get_vm_alignment(&self, device_id: u32) -> u32 {
        let page_size = self.PAGE_SIZE();

        if device_id >= 0x6920 && device_id <= 0x6939 {
            /* Tonga */
            // page_size = TONGA_PAGE_SIZE;
        } else if device_id >= 0x9870 && device_id <= 0x9877 {
            /* Carrizo */
            // page_size = TONGA_PAGE_SIZE;
        } else {
            // println!("device_id no apply get_vm_alignment {}", device_id);
        }

        // MAX(PAGE_SIZE, page_size);
        // MAX tmp1 > tmp2 ? tmp1 : tmp2
        page_size as u32
    }

    pub unsafe fn get_process_apertures(
        &self,
        process_apertures: *mut kfd_process_device_apertures,
        num_of_nodes: &mut u32,
    ) -> HsakmtStatus {
        let kfd_process_device_apertures_ptr = process_apertures as *mut u64;

        let mut args_new = kfd_ioctl_get_process_apertures_new_args {
            kfd_process_device_apertures_ptr,
            num_of_nodes: *num_of_nodes,
            pad: 0,
        };

        let p_1 = ('K' as i32) << 8;
        let p_2 =
            ((std::mem::size_of::<kfd_ioctl_get_process_apertures_new_args>()) << (8 + 8)) as i32;
        let AMDKFD_IOC_GET_PROCESS_APERTURES_NEW =
            (((2 | 1) << ((8 + 8) + 14)) | p_1 | (0x14)) | p_2;

        let hsakmt_kfd_fd = self.hsakmt_kfd_fd;

        let ret = hsakmt_ioctl(
            hsakmt_kfd_fd,
            AMDKFD_IOC_GET_PROCESS_APERTURES_NEW as u64,
            &mut args_new as *mut _ as *mut std::os::raw::c_void,
        );

        // println!("Hsakmt_ioctl returned {:?}", ret);
        if ret > 0 {
            println!(
                "hsakmt_kfd_fd {}, num_of_nodes {}",
                hsakmt_kfd_fd, num_of_nodes
            );
            panic!("hsakmt_ioctl failed {}", ret);
            // return HSAKMT_STATUS_ERROR
        }

        *num_of_nodes = args_new.num_of_nodes;
        HSAKMT_STATUS_SUCCESS
    }

    pub unsafe fn hsakmt_fmm_init_process_apertures(&mut self, NumNodes: u32) -> HsakmtStatus {
        let guardPages: u32 = 1;

        let zero_str = CString::new("0").unwrap();

        /* If HSA_DISABLE_CACHE is set to a non-0 value, disable caching */
        let env_str = CString::new("HSA_DISABLE_CACHE").unwrap();
        let disableCache = getenv(env_str.as_ptr());
        let b = !disableCache.is_null() && strcmp(disableCache, zero_str.as_ptr()) == 0;
        self.fmm.svm.disable_cache = b;

        /* If HSA_USERPTR_FOR_PAGED_MEM is not set or set to a non-0
         * value, enable userptr for all paged memory allocations
         */
        // let env_str = CString::new("HSA_USERPTR_FOR_PAGED_MEM").unwrap();
        // let pagedUserptr = getenv(env_str.as_ptr());
        // svm.userptr_for_paged_mem = !pagedUserptr.is_null() || strcmp(pagedUserptr, zero_str.as_ptr()) == 0;

        /* If HSA_CHECK_USERPTR is set to a non-0 value, check all userptrs
         * when they are registered
         */
        let env_str = CString::new("HSA_CHECK_USERPTR").unwrap();
        let checkUserptr = getenv(env_str.as_ptr());
        self.fmm.svm.check_userptr =
            !checkUserptr.is_null() && strcmp(checkUserptr, zero_str.as_ptr()) == 0;

        /* If HSA_RESERVE_SVM is set to a non-0 value,
         * enable packet capture and replay mode.
         */
        let env_str = CString::new("HSA_RESERVE_SVM").unwrap();
        let reserveSvm = getenv(env_str.as_ptr());
        self.fmm.svm.reserve_svm =
            !reserveSvm.is_null() && strcmp(reserveSvm, zero_str.as_ptr()) == 0;

        // let format_cs = CString::new("%u").unwrap();

        /* Specify number of guard pages for SVM apertures, default is 1 */
        // let env_str = CString::new("HSA_SVM_GUARD_PAGES").unwrap();
        // let guardPagesStr = getenv(env_str.as_ptr());
        // if !guardPagesStr.is_null() || sscanf(guardPagesStr, format_cs.as_ptr(), &guardPages) != 1 {
        //     guardPages = 1;
        // }

        /* Sets the max VA alignment order size during mapping. By default the order
         * size is set to 9(2MB)
         */
        // let env_str = CString::new("HSA_MAX_VA_ALIGN").unwrap();
        // let maxVaAlignStr = getenv(env_str.as_ptr());
        // if !maxVaAlignStr.is_null() || sscanf(maxVaAlignStr, format_cs.as_ptr(), &svm.alignment_order) != 1 {
        //     svm.alignment_order = 9;
        // }
        self.fmm.svm.alignment_order = 9;

        /* Initialize gpu_mem[] from sysfs topology. Rest of the members are
         * set to 0 by calloc. This is necessary because this function
         * gets called before hsaKmtAcquireSystemProperties() is called.
         */

        #[allow(clippy::field_reassign_with_default)]
        for i in 0..NumNodes {
            let mut KFDGpuID = 0;
            let mut DrmRenderMinor = 0;

            let mut Major = 0;
            let mut Minor = 0;
            let mut Stepping = 0;
            let mut LocalMemSize = 0;
            let mut DeviceId = 0;

            let mut NumCPUCores = 0;
            let mut NumFComputeCores = 0;

            let hsakmt_is_svm_api_supported = {
                let props = self.hsakmt_topology_get_node_props(i);
                // self.hsakmt_topology_setup_is_dgpu_param(props);
                // self.hsakmt_topology_setup_is_dgpu_param_v2(props);

                KFDGpuID = props.KFDGpuID;
                DrmRenderMinor = props.DrmRenderMinor;
                Major = props.EngineId.ui32.Major;
                Minor = props.EngineId.ui32.Minor;
                Stepping = props.EngineId.ui32.Stepping;
                LocalMemSize = props.LocalMemSize;
                DeviceId = props.DeviceId;

                NumCPUCores = props.NumCPUCores;
                NumFComputeCores = props.NumFComputeCores;

                props.Capability.ui32.SVMAPISupported > 0
            };

            self.hsakmt_topology_setup_is_dgpu_param_v2(DeviceId, NumCPUCores, NumFComputeCores);

            /* Skip non-GPU nodes */
            if KFDGpuID > 0 {
                let fd = self.hsakmt_open_drm_render_device(DrmRenderMinor);
                if fd <= 0 {
                    return HSAKMT_STATUS_ERROR;
                }

                let mut gpu_m = gpu_mem_t::default();

                gpu_m.drm_render_minor = DrmRenderMinor as u32;
                gpu_m.usable_peer_id_array.push(KFDGpuID);
                gpu_m.usable_peer_id_num = 1;

                gpu_m.EngineId.ui32.Major = Major;
                gpu_m.EngineId.ui32.Minor = Minor;
                gpu_m.EngineId.ui32.Stepping = Stepping;

                gpu_m.drm_render_fd = fd;
                gpu_m.gpu_id = KFDGpuID;
                gpu_m.local_mem_size = LocalMemSize;
                gpu_m.device_id = DeviceId as u32;
                gpu_m.node_id = i;

                self.hsakmt_is_svm_api_supported = hsakmt_is_svm_api_supported;

                gpu_m.scratch_physical.align = self.PAGE_SIZE() as u64;
                gpu_m.scratch_physical.ops = manageable_aperture_ops_t {
                    allocate_area_aligned: None,
                    release_area: None,
                };

                gpu_m.gpuvm_aperture.align = self.get_vm_alignment(DeviceId as u32) as u64;
                gpu_m.gpuvm_aperture.guard_pages = guardPages;
                gpu_m.gpuvm_aperture.ops = manageable_aperture_ops_t {
                    allocate_area_aligned: None,
                    release_area: None,
                };

                self.fmm.gpu_mem.push(gpu_m);
            }
        }

        /* The ioctl will also return Number of Nodes if
         * args.kfd_process_device_apertures_ptr is set to NULL. This is not
         * required since Number of nodes is already known. Kernel will fill in
         * the apertures in kfd_process_device_apertures_ptr
         */
        let mut num_of_sysfs_nodes = self.topology.num_sysfs_nodes as u32;
        if num_of_sysfs_nodes < self.fmm.gpu_mem.len() as u32 {
            return HSAKMT_STATUS_ERROR;
        }

        let mut process_apertures =
            vec![kfd_process_device_apertures::default(); num_of_sysfs_nodes as usize];

        /* GPU Resource management can disable some of the GPU nodes.
         * The Kernel driver could be not aware of this.
         * Get from Kernel driver information of all the nodes and then filter it.
         */
        let ret =
            self.get_process_apertures(process_apertures.as_mut_ptr(), &mut num_of_sysfs_nodes);
        if ret != HSAKMT_STATUS_SUCCESS {
            return ret;
        }

        process_apertures.pop();
        // println!("process_apertures {:#?}", process_apertures);

        let mut svm_base: u64 = 0;
        let mut svm_limit: u64 = 0;
        let mut svm_alignment: u32 = 0;

        // let mut all_gpu_id_array: Vec<u32> = Vec::with_capacity(self.fmm.gpu_mem.len());

        // for i in 0..num_of_sysfs_nodes as usize {
        //     /* Map Kernel process device data node i <--> gpu_mem_id which
        //      * indexes into gpu_mem[] based on gpu_id
        //      */
        //     let gpu_mem_id = self.gpu_mem_find_by_gpu_id(process_apertures[i].gpu_id);
        //
        //     // println!("gpu_mem_id i {} - {}", i, gpu_mem_id);
        //
        //     if gpu_mem_id < 0 {
        //         continue;
        //     }
        //
        //     let gpu_mem_id = gpu_mem_id as usize;
        //
        //     self.fmm.all_gpu_id_array.push(gpu_mem_id as u32);
        //
        //     /* Add this GPU to the usable_peer_id_arrays of all GPUs that
        //      * this GPU has an IO link to. This GPU can map memory
        //      * allocated on those GPUs.
        //      */
        //     let nodeId = self.fmm.gpu_mem[gpu_mem_id].node_id;
        //     let nodeProps = self.hsakmt_topology_get_node_props(nodeId);
        //
        //     assert!(nodeProps.NumIOLinks <= NumNodes);
        //     let linkProps: Vec<u32> = self
        //         .hsakmt_topology_get_iolink_props(nodeId)
        //         .iter()
        //         .map(|x| x.NodeTo)
        //         .collect();
        //
        //     for NodeTo in linkProps {
        //         let to_gpu_mem_id = self.gpu_mem_find_by_gpu_id(NodeTo);
        //
        //         if to_gpu_mem_id < 0 {
        //             continue;
        //         }
        //
        //         assert!(self.fmm.gpu_mem[to_gpu_mem_id as usize].usable_peer_id_num < NumNodes);
        //         let peer = self.fmm.gpu_mem[to_gpu_mem_id as usize].usable_peer_id_num;
        //
        //         self.fmm.gpu_mem[to_gpu_mem_id as usize].usable_peer_id_num += 1;
        //         self.fmm.gpu_mem[to_gpu_mem_id as usize].usable_peer_id_array[peer as usize] =
        //             self.fmm.gpu_mem[gpu_mem_id].gpu_id;
        //     }
        //
        //     self.fmm.gpu_mem[gpu_mem_id].lds_aperture.base =
        //         process_apertures[i].lds_base as *mut std::os::raw::c_void;
        //     self.fmm.gpu_mem[gpu_mem_id].lds_aperture.limit =
        //         process_apertures[i].lds_limit as *mut std::os::raw::c_void;
        //
        //     self.fmm.gpu_mem[gpu_mem_id].scratch_aperture.base =
        //         process_apertures[i].scratch_base as *mut std::os::raw::c_void;
        //     self.fmm.gpu_mem[gpu_mem_id].scratch_aperture.limit =
        //         process_apertures[i].scratch_limit as *mut std::os::raw::c_void;
        //
        //     if IS_CANONICAL_ADDR(process_apertures[i].gpuvm_limit) {
        //         let vm_alignment = self.get_vm_alignment(self.fmm.gpu_mem[gpu_mem_id].device_id);
        //
        //         /* Set proper alignment for scratch backing aperture */
        //         self.fmm.gpu_mem[gpu_mem_id].scratch_physical.align = vm_alignment as u64;
        //
        //         /* Non-canonical per-ASIC GPUVM aperture does
        //          * not exist on dGPUs in GPUVM64 address mode
        //          */
        //         self.fmm.gpu_mem[gpu_mem_id].gpuvm_aperture.base = std::ptr::null_mut();
        //         self.fmm.gpu_mem[gpu_mem_id].gpuvm_aperture.limit = std::ptr::null_mut();
        //
        //         /* Update SVM aperture limits and alignment */
        //         if process_apertures[i].gpuvm_base > svm_base {
        //             svm_base = process_apertures[i].gpuvm_base;
        //         }
        //         if process_apertures[i].gpuvm_limit < svm_limit || svm_limit == 0 {
        //             svm_limit = process_apertures[i].gpuvm_limit;
        //         }
        //         if vm_alignment > svm_alignment {
        //             svm_alignment = vm_alignment;
        //         }
        //     } else {
        //         self.fmm.gpu_mem[gpu_mem_id].gpuvm_aperture.base =
        //             process_apertures[i].gpuvm_base as *mut std::os::raw::c_void;
        //         self.fmm.gpu_mem[gpu_mem_id].gpuvm_aperture.limit =
        //             process_apertures[i].gpuvm_limit as *mut std::os::raw::c_void;
        //
        //         let g_args = HsakmtGlobalsArgs {
        //             page_size: self.PAGE_SIZE(),
        //             fmm_svm_alignment_order: self.fmm.svm.alignment_order,
        //         };
        //         /* Reserve space at the start of the
        //          * aperture. After subtracting the base, we
        //          * don't want valid pointers to become NULL.
        //          */
        //         aperture_allocate_area(
        //             &self.fmm.gpu_mem[gpu_mem_id].gpuvm_aperture,
        //             std::ptr::null_mut(),
        //             self.fmm.gpu_mem[gpu_mem_id].gpuvm_aperture.align,
        //             g_args,
        //         );
        //     }
        //
        //     /* Acquire the VM from the DRM render node for KFD use */
        //     let ret = self.acquire_vm(
        //         self.fmm.gpu_mem[gpu_mem_id].gpu_id,
        //         self.fmm.gpu_mem[gpu_mem_id].drm_render_fd,
        //     );
        //     if ret != HSAKMT_STATUS_SUCCESS {
        //         return ret;
        //     }
        // }
        //
        // self.fmm.all_gpu_id_array_size =
        //     (self.fmm.all_gpu_id_array.len() * std::mem::size_of::<u32>()) as u32;
        //
        // if svm_limit > 0 {
        //     /* At least one GPU uses GPUVM in canonical address
        //      * space. Set up SVM apertures shared by all such GPUs
        //      */
        //     let ret = self.init_svm_apertures(svm_base, svm_limit, svm_alignment, guardPages);
        //     if ret != HSAKMT_STATUS_SUCCESS {
        //         println!("init_svm_apertures error");
        //         return ret;
        //     }
        //
        //     // println!("init_svm_apertures continue");
        //
        //     for process_aperture in process_apertures.iter() {
        //         if !IS_CANONICAL_ADDR(process_aperture.gpuvm_limit) {
        //             continue;
        //         }
        //
        //         /* Set memory policy to match the SVM apertures */
        //         // let alt_base = svm.dgpu_alt_aperture_get_mut().unwrap();
        //         let alt_base = &mut self.fmm.svm.apertures[SVM_DEFAULT as usize];
        //
        //         let alt_size = VOID_PTRS_SUB(alt_base.limit, alt_base.base) + 1;
        //
        //         let d_c = if self.fmm.svm.disable_cache {
        //             KFD_IOC_CACHE_POLICY_COHERENT
        //         } else {
        //             KFD_IOC_CACHE_POLICY_NONCOHERENT
        //         };
        //
        //         let a_b = alt_base as *mut _ as *mut std::os::raw::c_void;
        //
        //         let err = self.fmm_set_memory_policy(
        //             process_aperture.gpu_id,
        //             d_c as i32,
        //             KFD_IOC_CACHE_POLICY_COHERENT as i32,
        //             a_b as *mut u64,
        //             alt_size,
        //         );
        //
        //         if err > 0 {
        //             println!(
        //                 "Failed to set mem policy for GPU {} {}",
        //                 process_aperture.gpu_id, err
        //             );
        //             return HSAKMT_STATUS_ERROR;
        //         }
        //     }
        // }
        //
        // let page_size = self.PAGE_SIZE();
        //
        // self.fmm.cpuvm_aperture.align = page_size as u64;
        // self.fmm.cpuvm_aperture.limit = 0x7FFFFFFFFFFF as *mut std::os::raw::c_void; /* 2^47 - 1 */
        //
        // // self.fmm_init_rbtree();
        //
        // if !self.init_mem_handle_aperture(page_size as u32, guardPages) {
        //     println!("Failed to init mem_handle_aperture\n");
        // }
        //
        // let hsakmt_kfd_fd = self.hsakmt_kfd_fd;
        //
        // let gpu_mem_count = self.fmm.gpu_mem.len();
        //
        // for i in 0..gpu_mem_count {
        //     let b = self.hsakmt_topology_is_svm_needed(&self.fmm.gpu_mem[i].EngineId);
        //
        //     if !b {
        //         // println!("hsakmt_topology_is_svm_needed no {}", b);
        //         continue;
        //     }
        //
        //     // println!("hsakmt_topology_is_svm_needed yes {}", b);
        //
        //     let r = self.map_mmio(
        //         self.fmm.gpu_mem[i].node_id,
        //         self.fmm.gpu_mem[i].gpu_id,
        //         hsakmt_kfd_fd,
        //         i,
        //     );
        //     // println!("map_mmio r {}", r.is_null());
        //
        //     self.fmm.gpu_mem[i].mmio_aperture.base = r;
        //
        //     if !self.fmm.gpu_mem[i].mmio_aperture.base.is_null() {
        //         let pt = (self.fmm.gpu_mem[i].mmio_aperture.base as *mut u8)
        //             .add((page_size - 1) as usize);
        //         let r = pt.add((page_size - 1) as usize);
        //
        //         self.fmm.gpu_mem[i].mmio_aperture.limit = r as *mut std::os::raw::c_void;
        //     } else {
        //         // println!("Failed to map remapped mmio page on gpu_mem {}", g_m.gpu_id);
        //         panic!(
        //             "Failed to map remapped mmio page on gpu_mem {}",
        //             self.fmm.gpu_mem[i].gpu_id
        //         );
        //     }
        // }

        HSAKMT_STATUS_SUCCESS
    }
}
