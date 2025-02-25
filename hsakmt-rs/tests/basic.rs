use hsakmt_rs::globals::HsakmtGlobals;
use hsakmt_rs::hsakmttypes::_HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS;

#[test]
fn test_basic() {
    let mut hsakmt = HsakmtGlobals::new();

    unsafe {
        let ret = hsakmt.hsaKmtOpenKFD();
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);

        let version_info = hsakmt.hsaKmtGetVersion();
        println!("version_info: {:?}", version_info);
        // assert_ne!(
        //     version_info,
        //     HsaVersionInfo {
        //         KernelInterfaceMajorVersion: 0,
        //         KernelInterfaceMinorVersion: 0
        //     }
        // );

        let ret = hsakmt.hsaKmtCloseKFD();
        assert_eq!(ret, _HSAKMT_STATUS_HSAKMT_STATUS_SUCCESS);
    }
}
