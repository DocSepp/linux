// SPDX-License-Identifier: GPL-2.0

use kernel::prelude::*;

module! {
    type: MyRustDriver,
    name: "my_rust_driver",
    author: "Sebastian Weyer",
    description: "Rust character device printing hello message on use",
    license: "GPL v2",
}

struct MyRustDriver;

impl kernel::Module for MyRustDriver {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("My custom Rust character device (init)\n");
        pr_info!("Hello from the rustacean world! ? \n");
        Ok(Self)
    }
}

impl Drop for MyRustDriver {
    fn drop(&mut self) {
        pr_info!("Bye!\n");
        pr_info!("My custom Rust character device (exit)\n");
    }
}
