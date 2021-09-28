// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

module! {

}

struct Tty0tty;


impl KernelModule for Tty0tty {
    fn init() -> Result<Self> {
        pr_info!("Hello, World!");

        Ok(Tty0tty {})
    }
}

impl Drop for Tty0tty {
    fn drop(&mut self) {
        pr_info("Goodbye, World!");
    }
}
