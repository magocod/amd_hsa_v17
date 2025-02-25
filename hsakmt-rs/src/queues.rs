#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use crate::globals::HsakmtGlobals;
use crate::hsakmttypes::{_HSAKMT_STATUS, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS};

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

/* Calculate VGPR and SGPR register file size per CU */
pub const SGPR_SIZE_PER_CU: usize = 0x4000;

pub const GFX_VERSION_KAVERI: usize = 0x070000;
pub const GFX_VERSION_HAWAII: usize = 0x070001;
pub const GFX_VERSION_CARRIZO: usize = 0x080001;
pub const GFX_VERSION_TONGA: usize = 0x080002;
pub const GFX_VERSION_FIJI: usize = 0x080003;
pub const GFX_VERSION_POLARIS10: usize = 0x080003;
pub const GFX_VERSION_POLARIS11: usize = 0x080003;
pub const GFX_VERSION_POLARIS12: usize = 0x080003;
pub const GFX_VERSION_VEGAM: usize = 0x080003;
pub const GFX_VERSION_VEGA10: usize = 0x090000;
pub const GFX_VERSION_RAVEN: usize = 0x090002;
pub const GFX_VERSION_VEGA12: usize = 0x090004;
pub const GFX_VERSION_VEGA20: usize = 0x090006;
pub const GFX_VERSION_ARCTURUS: usize = 0x090008;
pub const GFX_VERSION_ALDEBARAN: usize = 0x09000A;
pub const GFX_VERSION_AQUA_VANJARAM: usize = 0x090400;
pub const GFX_VERSION_RENOIR: usize = 0x09000C;
pub const GFX_VERSION_NAVI10: usize = 0x0A0100;
pub const GFX_VERSION_NAVI12: usize = 0x0A0101;
pub const GFX_VERSION_NAVI14: usize = 0x0A0102;
pub const GFX_VERSION_CYAN_SKILLFISH: usize = 0x0A0103;
pub const GFX_VERSION_SIENNA_CICHLID: usize = 0x0A0300;
pub const GFX_VERSION_NAVY_FLOUNDER: usize = 0x0A0301;
pub const GFX_VERSION_DIMGREY_CAVEFISH: usize = 0x0A0302;
pub const GFX_VERSION_VANGOGH: usize = 0x0A0303;
pub const GFX_VERSION_BEIGE_GOBY: usize = 0x0A0304;
pub const GFX_VERSION_YELLOW_CARP: usize = 0x0A0305;
pub const GFX_VERSION_PLUM_BONITO: usize = 0x0B0000;
pub const GFX_VERSION_WHEAT_NAS: usize = 0x0B0001;
pub const GFX_VERSION_GFX1200: usize = 0x0C0000;
pub const GFX_VERSION_GFX1201: usize = 0x0C0001;

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
    pub fn hsakmt_init_process_doorbells(&mut self, NumNodes: u32) -> _HSAKMT_STATUS {
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

        _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS
    }
}
