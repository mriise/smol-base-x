use smol_base_x::{Base, Base58Btc};
fn main() {
    let src = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    // generated from https://www.dcode.fr/base-58-cipher
    let out = "6WWAVR6RaTut2Av6UM6awEwUE5NwgCpoRmC9WQmcjKLWSwQVE6rcRW23MBinCQ1xxPcFgZB9z2jp1igKVp1f6sdJxmf1c9GpMFxi4e1fp7zEJgJrFYD6yrVxqo2kfLAEV8xYYBJPGJTzkKMq7kfZXuTxnoNdPCjqsYDaCvsLsbwdNWgyHW6Ub9K1f5FXZTVobWAsRBNwaXmDRi78ZWz5h5fnUVRnPiq3HHvSu8DBqdxPngorx8rRkswtDsz1KbFyzDTE7W5eFYoAYbszBmkfR2CTHfoT4yZXYkU4YSLPnLGPZeEaMQonDjr3vN35aCcgeHiJq34kVbENgqet8n8cdh2phNEWyRS8ok6A62Ynb5qFnCVzuDqXYHKJCAyrqudpWS2zbRHEivNAe7B6WBuyPUg86mXZEgyGwsEiv517fWQL6hZcj4NfaqNpGsGJMgvUhu6MGgLruphbqQYEpZeLUk3zcfWqGHoVLW3iwi6i9ULDefXvVEU2SdtfkBQi7xGnZurxPxgShbofmx3QxVTLWntL7gB2LGQ2NWtEyUuxrE2h1UKeEDvPjC6dZpNdemDL8FiMQ15nSSnsEj6GEYaPScox6mjCvouw";

    let mut buf = [0u8; 1024];
    let written = Base58Btc::decode_mut(out, &mut buf).expect("should've been fine...");
    assert_eq!(src.as_bytes(), &buf[..written]);

    // functions expect empty buffer
    buf.fill(0);

    let written = Base58Btc::encode_mut(src, &mut buf).expect("should've been fine...");
    assert_eq!(out.as_bytes(), &buf[..written]);
}
