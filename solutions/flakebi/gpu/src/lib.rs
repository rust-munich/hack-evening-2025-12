#![feature(abi_gpu_kernel)]
#![no_std]

extern crate alloc;

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use amdgpu_device_libs::prelude::*;

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "gpu-kernel" fn kernel(
    input: *const u8,
    output: *mut u32,
    width: u32,
    height: u32,
) {
    let dispatch = dispatch_ptr();

    // Compute global coordinates
    let x = workgroup_id_x() * dispatch.workgroup_size_x as u32 + workitem_id_x();
    let y = workgroup_id_y() * dispatch.workgroup_size_y as u32 + workitem_id_y();

    if x >= width || y >= height {
        return;
    }

    // Read at coordinate
    let get = |x, y| unsafe { *input.add((y * (width + 1) + x) as usize) };
    let is_roll = |x, y| get(x, y) == b'@';
    let roll_num = |x, y| if is_roll(x, y) { 1 } else { 0 };

    let atomic = unsafe { AtomicU32::from_ptr(output) };
    if is_roll(x, y) {
        let mut set_neighbors = 0;
        if x > 0 {
            if y > 0 {
                set_neighbors += roll_num(x - 1, y - 1);
            }
            set_neighbors += roll_num(x - 1, y);
            if y < (width - 1) {
                set_neighbors += roll_num(x - 1, y + 1);
            }
        }
        if y > 0 {
            set_neighbors += roll_num(x, y - 1);
        }
        if y < (width - 1) {
            set_neighbors += roll_num(x, y + 1);
        }
        if x < (width - 1) {
            if y > 0 {
                set_neighbors += roll_num(x + 1, y - 1);
            }
            set_neighbors += roll_num(x + 1, y);
            if y < (width - 1) {
                set_neighbors += roll_num(x + 1, y + 1);
            }
        }
        if set_neighbors < 4 {
            atomic.fetch_add(1, Ordering::Relaxed);
        }
    }
}
