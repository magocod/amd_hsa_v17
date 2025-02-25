#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

use crate::hsakmttypes::{
    HsaCacheProperties, HsaIoLinkProperties, HsaMemoryProperties, HsaNodeProperties,
    HSA_CAPABILITY, HSA_DEBUG_PROPERTIES, HSA_ENGINE_ID, HSA_ENGINE_VERSION,
};

pub struct node_props_t {
    pub node: HsaNodeProperties,
    pub mem: Vec<HsaMemoryProperties>, /* node->NumBanks elements */
    pub cache: Vec<HsaCacheProperties>,
    pub link: Vec<HsaIoLinkProperties>,
}

impl node_props_t {
    pub fn new() -> Self {
        Self {
            node: HsaNodeProperties {
                NumCPUCores: 0,
                NumFComputeCores: 0,
                NumNeuralCores: 0,
                NumMemoryBanks: 0,
                NumCaches: 0,
                NumIOLinks: 0,
                CComputeIdLo: 0,
                FComputeIdLo: 0,
                Capability: HSA_CAPABILITY { Value: 0 },
                MaxWavesPerSIMD: 0,
                LDSSizeInKB: 0,
                GDSSizeInKB: 0,
                WaveFrontSize: 0,
                NumShaderBanks: 0,
                NumArrays: 0,
                NumCUPerArray: 0,
                NumSIMDPerCU: 0,
                MaxSlotsScratchCU: 0,
                EngineId: HSA_ENGINE_ID { Value: 0 },
                OverrideEngineId: HSA_ENGINE_ID { Value: 0 },
                VendorId: 0,
                DeviceId: 0,
                LocationId: 0,
                LocalMemSize: 0,
                MaxEngineClockMhzFCompute: 0,
                MaxEngineClockMhzCCompute: 0,
                DrmRenderMinor: 0,
                MarketingName: [0; 64],
                AMDName: [0; 64],
                uCodeEngineVersions: HSA_ENGINE_VERSION { Value: 0 },
                DebugProperties: HSA_DEBUG_PROPERTIES { Value: 0 },
                HiveID: 0,
                NumSdmaEngines: 0,
                NumSdmaXgmiEngines: 0,
                NumSdmaQueuesPerEngine: 0,
                NumCpQueues: 0,
                NumGws: 0,
                Integrated: 0,
                Domain: 0,
                UniqueID: 0,
                VGPRSizePerCU: 0,
                SGPRSizePerCU: 0,
                NumXcc: 0,
                KFDGpuID: 0,
                FamilyID: 0,
            },
            mem: vec![],
            cache: vec![],
            link: vec![],
        }
    }
}

impl Default for node_props_t {
    fn default() -> Self {
        Self::new()
    }
}
