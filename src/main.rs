#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rainbows2::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use rainbows2::{println};
use x86_64::VirtAddr;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use rainbows2::task::{Task, executor::Executor, keyboard};


// add a `config` argument to the `entry_point` macro call
entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rainbows2::memory::{self, BootInfoFrameAllocator};
    use rainbows2::allocator;

    println!("Rainbows OS 0.1-dev.02-02-2023--13-07-40");

    rainbows2::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new(); // new
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rainbows2::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rainbows2::test_panic_handler(info)
}
