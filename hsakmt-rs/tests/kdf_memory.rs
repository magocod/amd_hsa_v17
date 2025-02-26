// use std::thread;
use hsakmt_rs::rbtree::print_tree;
use hsakmt_rs::test_kfd_utils::kfd_base_component::KFDBaseComponentTest;

#[test]
fn test_base_component() {
    let mut kfd_base = KFDBaseComponentTest::new();

    unsafe {
        kfd_base.set_up();
    }

    println!(
        "kfd_base.hsakmt.topology.g_props.len = {:?}",
        kfd_base.hsakmt.topology.g_props.len()
    );
    println!(
        "kfd_base.hsakmt.topology.gpu_mem.len = {:?}",
        kfd_base.hsakmt.fmm.gpu_mem.len()
    );
    println!(
        "kfd_base.hsakmt.queue.doorbells.len = {:?}",
        kfd_base.hsakmt.queue.doorbells.len()
    );
    
    println!("global size = {}", std::mem::size_of_val(&kfd_base));

    for aperture in kfd_base.hsakmt.fmm.svm.apertures.iter() {
        println!("---");

        unsafe {
            println!("aperture.tree");
            print_tree(&aperture.tree);
            println!();

            println!("aperture.user_tree");
            print_tree(&aperture.user_tree);
            println!();
        }

        println!("---");
    }

    // println!("assert test");
}

// Basic test of hsaKmtMapMemoryToGPU and hsaKmtUnmapMemoryToGPU
// #[test]
// fn test_map_memory_to_gpu() {
//     let mut kfd_base = KFDBaseComponentTest::new();
//
//     let mut pDb: *mut std::os::raw::c_void = std::ptr::null_mut();
//
//     let gpu_node = 1;
//     let page_size = 1 << 12;
//
//     unsafe {
//         kfd_base.set_up();
//
//         let ret = kfd_base.hsakmt.hsaKmtAllocMemory(
//             gpu_node,
//             page_size,
//             kfd_base.m_MemoryFlags,
//             &mut pDb,
//             TEST_MAP_MEMORY_TO_GPU_VECTOR_INDEX,
//         );
//         assert_eq!(ret, HSAKMT_STATUS_SUCCESS);
//
//         // verify that pDb is not null before it's being used
//         assert_ne!(pDb, std::ptr::null_mut());
//
//         // let ret = kfd_base.hsakmt.hsaKmtMapMemoryToGPU(
//         //     pDb,
//         //     page_size,
//         //     std::ptr::null_mut(),
//         //     TEST_MAP_MEMORY_TO_GPU_VECTOR_INDEX,
//         // );
//         // assert_eq!(ret, HSAKMT_STATUS_SUCCESS);
//
//         let ret = kfd_base.hsakmt.hsaKmtMapMemoryToGPU(
//             pDb,
//             page_size,
//             std::ptr::null_mut(),
//             TEST_MAP_MEMORY_TO_GPU_VECTOR_INDEX,
//         );
//         assert_eq!(ret, HSAKMT_STATUS_SUCCESS);
//
//         for aperture in kfd_base.hsakmt.fmm.svm.apertures.iter() {
//             let map_len = aperture.tree.len();
//             println!();
//             println!("aperture.tree (HashMap) = {:?}", map_len);
//
//             for i in 0..aperture.tree.iter().len() {
//                 println!("vm_object.is_null = {:#?}", aperture.tree[i].is_null());
//                 if !aperture.tree[i].is_null() {
//                     let vm_object = &(*aperture.tree[i]);
//                     println!("vm_object.node = {:#?}", vm_object.node);
//                     println!("vm_object.user_node = {:#?}", vm_object.user_node);
//                 }
//             }
//
//             println!();
//         }
//     }
// }
//
// // Basic test for hsaKmtAllocMemory
// #[test]
// fn test_basic_hsakmt_alloc_memory() {
//     let mut kfd_base = KFDBaseComponentTest::new();
//
//     let mut pDb: *mut std::os::raw::c_void = std::ptr::null_mut();
//
//     let gpu_node = 1;
//     let page_size = 1 << 12;
//
//     unsafe {
//         kfd_base.set_up();
//
//         kfd_base.m_MemoryFlags.st.ui32.NoNUMABind = 1;
//
//         let ret = kfd_base.hsakmt.hsaKmtAllocMemory(
//             gpu_node,
//             page_size,
//             kfd_base.m_MemoryFlags,
//             &mut pDb,
//             TEST_MAP_MEMORY_TO_GPU_VECTOR_INDEX,
//         );
//         assert_eq!(ret, HSAKMT_STATUS_SUCCESS);
//         assert_ne!(pDb, std::ptr::null_mut());
//
//         for aperture in kfd_base.hsakmt.fmm.svm.apertures.iter() {
//             let map_len = aperture.tree.len();
//             println!();
//             println!("aperture.tree (HashMap) = {:?}", map_len);
//
//             for i in 0..aperture.tree.iter().len() {
//                 println!("vm_object.is_null = {:#?}", aperture.tree[i].is_null());
//                 if !aperture.tree[i].is_null() {
//                     let vm_object = &(*aperture.tree[i]);
//                     println!("vm_object.node = {:#?}", vm_object.node);
//                     println!("vm_object.user_node = {:#?}", vm_object.user_node);
//                 }
//             }
//
//             println!();
//         }
//     }
// }
//
// // Basic test for hsaKmtAllocMemory
// #[test]
// fn test_basic_hsakmt_memory_alloc_all() {
//     let mut kfd_base = KFDBaseComponentTest::new();
//
//     // let mut pDb: *mut std::os::raw::c_void = std::ptr::null_mut();
//
//     let gpu_node = 1;
//
//     let mut available = 0;
//
//     unsafe {
//         kfd_base.set_up();
//
//         let ret = kfd_base
//             .hsakmt
//             .hsaKmtAvailableMemory(gpu_node, &mut available);
//         assert_eq!(ret, HSAKMT_STATUS_SUCCESS);
//
//         println!("available = {:#?} bytes", available);
//
//         // for aperture in kfd_base.hsakmt.fmm.svm.apertures.iter() {
//         //     let map_len = aperture.tree.len();
//         //     println!();
//         //     println!("aperture.tree (HashMap) = {:?}", map_len);
//         //
//         //     for i in 0..aperture.tree.iter().len() {
//         //         println!("vm_object.is_null = {:#?}", aperture.tree[i].is_null());
//         //         if !aperture.tree[i].is_null() {
//         //             let vm_object = &(*aperture.tree[i]);
//         //             println!("vm_object.node = {:#?}", vm_object.node);
//         //             println!("vm_object.user_node = {:#?}", vm_object.user_node);
//         //         }
//         //     }
//         //
//         //     println!();
//         // }
//     }
// }
