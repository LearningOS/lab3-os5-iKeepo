#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use alloc::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use sync::UPSafeCell;
use task::add_task;

use crate::task::Task;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

extern crate alloc;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod logging;
mod mm;
mod sbi;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

core::arch::global_asm!(include_str!("entry.asm"));
core::arch::global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    mm::init();
    info!("after mm init!");
    mm::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    loader::list_apps();
    // task::add_initproc();
    // info!("after initproc!");
    run_usertest();
    task::run_next_task()
}
lazy_static! {
    pub static ref BATCH_PROCESSING_TASK: UPSafeCell<Vec<Arc<Task>>> =
        unsafe { UPSafeCell::new(Vec::new()) };
}

pub fn run_target_task(names: &[&str]) {
    let mut batch_processing_task = BATCH_PROCESSING_TASK.exclusive_access();
    for name in names.iter() {
        batch_processing_task.push(Task::new(*name));
    }

    for task in batch_processing_task.iter() {
        add_task(Arc::clone(task))
    }
}

pub fn run_usertest() {
    run_target_task(&["ch5_usertest"]);
}