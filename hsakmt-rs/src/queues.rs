#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use crate::globals::HsakmtGlobals;
use crate::hsakmttypes::HsakmtStatus::HSAKMT_STATUS_SUCCESS;
use crate::hsakmttypes::{
    HsakmtStatus, GFX_VERSION_ALDEBARAN, GFX_VERSION_AQUA_VANJARAM, GFX_VERSION_ARCTURUS,
    GFX_VERSION_GFX1200, GFX_VERSION_GFX1201, GFX_VERSION_PLUM_BONITO, GFX_VERSION_WHEAT_NAS,
};

#[derive(Debug)]
pub struct queue {
    queue_id: u32,
    wptr: u64,
    rptr: u64,
    eop_buffer: *mut std::os::raw::c_void,
    ctx_save_restore: *mut std::os::raw::c_void,
    ctx_save_restore_size: u32,
    ctl_stack_size: u32,
    debug_memory_size: u32,
    eop_buffer_size: u32,
    total_mem_alloc_size: u32,
    gfxv: u32,
    use_ats: bool,
    unified_ctx_save_restore: bool,
    /* This queue structure is allocated from GPU with page aligned size
     * but only small bytes are used. We use the extra space in the end for
     * cu_mask bits array.
     */
    cu_mask_count: u32, /* in bits */
    cu_mask: Vec<u32>,
}

#[derive(Debug)]
pub struct process_doorbells {
    use_gpuvm: bool,
    size: u32,
    mapping: *mut std::os::raw::c_void,
}

pub fn hsakmt_get_vgpr_size_per_cu(gfxv: u32) -> u32 {
    let mut vgpr_size = 0x40000;

    if (gfxv & !0xff) == GFX_VERSION_AQUA_VANJARAM as u32
        || gfxv == GFX_VERSION_ALDEBARAN as u32
        || gfxv == GFX_VERSION_ARCTURUS as u32
    {
        vgpr_size = 0x80000;
    } else if gfxv == GFX_VERSION_PLUM_BONITO as u32
        || gfxv == GFX_VERSION_WHEAT_NAS as u32
        || gfxv == GFX_VERSION_GFX1200 as u32
        || gfxv == GFX_VERSION_GFX1201 as u32
    {
        vgpr_size = 0x60000;
    }

    vgpr_size
}

impl HsakmtGlobals {
    pub fn hsakmt_init_process_doorbells(&mut self, NumNodes: u32) -> HsakmtStatus {
        /* doorbells[] is accessed using Topology NodeId. This means doorbells[0],
         * which corresponds to CPU only Node, might not be used
         */

        for _ in 0..NumNodes {
            let p = process_doorbells {
                use_gpuvm: false,
                size: 0,
                mapping: std::ptr::null_mut(),
            };
            self.queue.doorbells.push(p);
        }

        self.queue.num_doorbells = NumNodes;

        HSAKMT_STATUS_SUCCESS
    }
}
