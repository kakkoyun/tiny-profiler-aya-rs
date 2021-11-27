#![no_std]
#![no_main]

use aya_bpf::{
    macros::tracepoint,
    programs::TracePointContext,
};

#[tracepoint(name="tiny_profiler")]
pub fn tiny_profiler(ctx: TracePointContext) -> u32 {
    match unsafe { try_tiny_profiler(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_tiny_profiler(_ctx: TracePointContext) -> Result<u32, u32> {
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
