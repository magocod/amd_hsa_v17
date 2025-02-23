#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

use crate::hsakmttypes::{HsaMemFlagUnion, HsaMemFlags, HSA_ENGINE_ID};
use crate::rbtree::{rbtree_node_t, rbtree_t};
use amdgpu_drm_sys::bindings::amdgpu_device;

#[derive(Debug, Clone, PartialEq)]
pub struct vm_area {
    pub start: *mut std::os::raw::c_void,
    pub end: *mut std::os::raw::c_void,
    pub next: *mut vm_area,
    pub prev: *mut vm_area,
}

pub type vm_area_t = vm_area;

pub struct HsakmtGlobalsArgs {
    pub page_size: i32,
    pub fmm_svm_alignment_order: u32,
}

/* Aperture management function pointers to allow different management
 * schemes.
 */
#[allow(clippy::type_complexity)]
#[derive(Debug, Clone, PartialEq)]
pub struct manageable_aperture_ops_t {
    // allocate_area_aligned: &'static fn(&[u8]) -> *mut std::os::raw::c_void,
    pub allocate_area_aligned: Option<
        unsafe fn(
            aper: &manageable_aperture_t,
            addr: *mut std::os::raw::c_void,
            size: u64,
            align: u64,
            hsakmt_global: HsakmtGlobalsArgs,
        ) -> *mut std::os::raw::c_void,
    >,
    pub release_area:
        Option<unsafe fn(aper: &manageable_aperture_t, addr: *mut std::os::raw::c_void, size: u64)>,
    // void *(*allocate_area_aligned)(manageable_aperture_t *aper, void *addr, uint64_t size, uint64_t align);
    // void (*release_area)(manageable_aperture_t *aper, void *addr, uint64_t size);
}

pub const TEST_MAP_MEMORY_TO_GPU_VECTOR_INDEX: usize = 2;

pub struct manageable_aperture {
    pub base: *mut std::os::raw::c_void,
    pub limit: *mut std::os::raw::c_void,
    pub align: u64,
    pub guard_pages: u32,
    pub vm_ranges: vm_area_t,
    pub tree: rbtree_t,
    pub user_tree: rbtree_t,
    pub is_cpu_accessible: bool,
    // ops: &'a manageable_aperture_ops_t,
    pub ops: manageable_aperture_ops_t,
}

impl PartialEq for manageable_aperture {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.limit == other.limit
            && self.align == other.align
            && self.guard_pages == other.guard_pages
            && self.vm_ranges == other.vm_ranges
            && self.is_cpu_accessible == other.is_cpu_accessible
            && self.ops == other.ops
        // && self.tree == other.tree
        // && self.user_tree == other.user_tree
    }
}

// pub const TREE_CAPACITY: usize = 4;

impl manageable_aperture {
    pub unsafe fn INIT_MANAGEABLE_APERTURE(base_value: usize, limit_value: usize) -> Self {
        let aperture = Self {
            base: base_value as *mut std::os::raw::c_void,
            limit: limit_value as *mut std::os::raw::c_void,
            align: 0,
            guard_pages: 1,
            vm_ranges: vm_area {
                start: std::ptr::null_mut(),
                end: std::ptr::null_mut(),
                next: std::ptr::null_mut(),
                prev: std::ptr::null_mut(),
            },
            // tree: Vec::new(),
            // user_tree: Vec::new(),
            tree: rbtree_t::init(),
            user_tree: rbtree_t::init(),
            is_cpu_accessible: false,
            ops: manageable_aperture_ops_t {
                allocate_area_aligned: None,
                release_area: None,
            },
        };

        // aperture.tree.sentinel.left = &mut aperture.tree.sentinel as *mut rbtree_node_t;
        // aperture.tree.sentinel.right = &mut aperture.tree.sentinel as *mut rbtree_node_t;

        aperture
    }
}

unsafe impl Send for manageable_aperture {}

/* Memory manager for an aperture */
pub type manageable_aperture_t = manageable_aperture;

pub enum svm_aperture_type {
    SVM_DEFAULT = 0,
    SVM_COHERENT,
    SVM_APERTURE_NUM,
}

/* The main structure for dGPU Shared Virtual Memory Management */
pub struct svm_t {
    /* Two apertures can have different MTypes (for coherency) */
    pub apertures: [manageable_aperture_t; 2],

    /* Pointers to apertures, may point to the same aperture on
     * GFXv9 and later, where MType is not based on apertures
     */
    // pub dgpu_aperture: Option<&'a manageable_aperture_t<'a>>,
    // pub dgpu_alt_aperture: Option<&'amanageable_aperture_t<'a>>,
    pub dgpu_aperture: *mut manageable_aperture_t,
    pub dgpu_alt_aperture: *mut manageable_aperture_t,

    /* whether to use userptr for paged memory */
    pub userptr_for_paged_mem: bool,

    /* whether to check userptrs on registration */
    pub check_userptr: bool,

    /* whether to check reserve svm on registration */
    pub reserve_svm: bool,

    /* whether all memory is coherent (GPU cache disabled) */
    pub disable_cache: bool,

    /* specifies the alignment size as PAGE_SIZE * 2^alignment_order */
    pub alignment_order: u32,
}

impl Default for svm_t {
    fn default() -> Self {
        unsafe {
            Self {
                apertures: [
                    manageable_aperture_t::INIT_MANAGEABLE_APERTURE(0, 0),
                    manageable_aperture_t::INIT_MANAGEABLE_APERTURE(0, 0),
                ],
                dgpu_aperture: std::ptr::null_mut(),
                dgpu_alt_aperture: std::ptr::null_mut(),
                userptr_for_paged_mem: false,
                check_userptr: false,
                reserve_svm: false,
                disable_cache: false,
                alignment_order: 0,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct aperture_t {
    pub base: *mut std::os::raw::c_void,
    pub limit: *mut std::os::raw::c_void,
}

pub struct gpu_mem_t {
    pub gpu_id: u32,
    pub device_id: u32,
    pub node_id: u32,
    pub local_mem_size: u64,
    pub EngineId: HSA_ENGINE_ID,
    pub lds_aperture: aperture_t,
    pub scratch_aperture: aperture_t,
    pub mmio_aperture: aperture_t,
    pub scratch_physical: manageable_aperture_t, /* For dGPU, scratch physical is allocated from
                                                  * dgpu_aperture. When requested by RT, each
                                                  * GPU will get a differnt range
                                                  */
    pub gpuvm_aperture: manageable_aperture_t, /* used for GPUVM on APU, outsidethe canonical address range */
    pub drm_render_fd: i32,
    pub usable_peer_id_num: u32,
    pub usable_peer_id_array: Vec<u32>,
    pub drm_render_minor: u32,
}

unsafe impl Send for gpu_mem_t {}

impl Default for gpu_mem_t {
    fn default() -> Self {
        unsafe {
            Self {
                gpu_id: 0,
                device_id: 0,
                node_id: 0,
                local_mem_size: 0,
                EngineId: HSA_ENGINE_ID { Value: 0 },
                lds_aperture: aperture_t {
                    base: std::ptr::null_mut(),
                    limit: std::ptr::null_mut(),
                },
                scratch_aperture: aperture_t {
                    base: std::ptr::null_mut(),
                    limit: std::ptr::null_mut(),
                },
                mmio_aperture: aperture_t {
                    base: std::ptr::null_mut(),
                    limit: std::ptr::null_mut(),
                },
                scratch_physical: manageable_aperture::INIT_MANAGEABLE_APERTURE(0, 0),
                gpuvm_aperture: manageable_aperture::INIT_MANAGEABLE_APERTURE(0, 0),
                drm_render_fd: 0,
                usable_peer_id_num: 0,
                usable_peer_id_array: vec![],
                drm_render_minor: 0,
            }
        }
    }
}

/* The VMs from DRM render nodes are used by KFD for the lifetime of
 * the process. Therefore we have to keep using the same FDs for the
 * lifetime of the process, even when we close and reopen KFD. There
 * are up to 128 render nodes that we cache in this array.
 */
pub const DRM_FIRST_RENDER_NODE: usize = 128;
pub const DRM_LAST_RENDER_NODE: usize = 255;

pub struct HsaKmtFmmGlobal {
    pub drm_render_fds: [i32; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
    pub amdgpu_handle: [amdgpu_device; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
    pub svm: svm_t,
    /* The other apertures are specific to each GPU. gpu_mem_t manages GPU
     * specific memory apertures.
     */
    pub gpu_mem: Vec<gpu_mem_t>,
    // pub gpu_mem_count: u32,
    // pub g_first_gpu_mem: gpu_mem_t,
    /* GPU node array for default mappings */
    pub all_gpu_id_array_size: u32,
    pub all_gpu_id_array: Vec<u32>,
    pub dgpu_shared_aperture_base: *mut std::os::raw::c_void,
    pub dgpu_shared_aperture_limit: *mut std::os::raw::c_void,
}

unsafe impl Send for HsaKmtFmmGlobal {}

impl Clone for HsaKmtFmmGlobal {
    fn clone(&self) -> Self {
        Self {
            drm_render_fds: self.drm_render_fds,
            amdgpu_handle: self.amdgpu_handle,
            svm: svm_t::default(),
            gpu_mem: vec![],
            all_gpu_id_array_size: 0,
            all_gpu_id_array: vec![],
            dgpu_shared_aperture_base: std::ptr::null_mut(),
            dgpu_shared_aperture_limit: std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct vm_object {
    pub start: *mut std::os::raw::c_void,
    pub userptr: *mut std::os::raw::c_void,
    pub userptr_size: u64,
    pub size: u64,        /* size allocated on GPU. When the user requests a random
                          	* size, Thunk aligns it to page size and allocates this
                          	* aligned size on GPU
                          	*/
    pub handle: *mut u64, /* opaque */
    pub node_id: u32,
    pub node: rbtree_node_t,
    pub user_node: rbtree_node_t,

    pub mflags: HsaMemFlags, /* memory allocation flags */
    /* Registered nodes to map on SVM mGPU */
    pub registered_device_id_array: *mut u32,
    pub registered_device_id_array_size: u32,
    pub registered_node_id_array: *mut u32,
    pub registration_count: u32, /* the same memory region can be registered multiple times */
    /* Nodes that mapped already */
    pub mapped_device_id_array: Vec<u32>,
    pub mapped_device_id_array_size: u32,
    pub mapped_node_id_array: *mut u32,
    pub mapping_count: u32,
    /* Metadata of imported graphics buffers */
    pub metadata: *mut std::os::raw::c_void,
    /* User data associated with the memory */
    pub user_data: *mut std::os::raw::c_void,
    /* Flag to indicate imported KFD buffer */
    pub is_imported_kfd_bo: bool,
}

pub type vm_object_t = vm_object;

impl Default for vm_object {
    fn default() -> Self {
        Self {
            start: std::ptr::null_mut(),
            userptr: std::ptr::null_mut(),
            userptr_size: 0,
            size: 0,
            handle: 0 as *mut u64,
            node_id: 0,
            node: Default::default(),
            user_node: Default::default(),
            mflags: HsaMemFlags {
                st: HsaMemFlagUnion { Value: 0 },
            },
            registered_device_id_array: std::ptr::null_mut(),
            registered_device_id_array_size: 0,
            registered_node_id_array: std::ptr::null_mut(),
            registration_count: 0,
            // mapped_device_id_array: std::ptr::null_mut(),
            mapped_device_id_array: vec![],
            mapped_device_id_array_size: 0,
            mapped_node_id_array: std::ptr::null_mut(),
            mapping_count: 0,
            metadata: std::ptr::null_mut(),
            user_data: std::ptr::null_mut(),
            is_imported_kfd_bo: false,
        }
    }
}
