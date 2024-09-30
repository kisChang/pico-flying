# WiFi + Bluetooth firmware blobs

Firmware obtained from https://github.com/georgerobotics/cyw43-driver/tree/main/firmware

Licensed under the [Infineon Permissive Binary License](./LICENSE-permissive-binary-license-1.0.txt)

## Changelog

* 2023-08-21: synced with `a1dc885` - Update 43439 fw + clm to come from `wb43439A0_7_95_49_00_combined.h` + add Bluetooth firmware
* 2023-07-28: synced with `ad3bad0` - Update 43439 fw from 7.95.55 to 7.95.62

## Notes

If you update these files, please update the lengths in the `tests/rp/src/bin/cyw43_perf.rs` test (which relies on these files running from RAM).


// To make flashing faster for development, you may want to flash the firmwares independently
// at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
//     probe-rs download 43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
//     probe-rs download 43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000
//let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
//let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

刷新网卡固件
probe-rs download 43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000 --probe 0d28:0204:0700000100340063520000154e525943a5a5a5a597969908
probe-rs download 43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000 --probe 0d28:0204:0700000100340063520000154e525943a5a5a5a597969908