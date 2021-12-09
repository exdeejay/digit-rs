use std::mem::{self, MaybeUninit};
use winapi::{
    shared::windef::{POINT, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
    um::{
        winuser::{
            GetMonitorInfoA, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONULL,
            MONITOR_DEFAULTTOPRIMARY, SetProcessDpiAwarenessContext
        },
    },
};

pub fn set_process_dpi_aware() {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
    };
}

pub fn get_monitorinfo(x: i32, y: i32) -> Option<MONITORINFO> {
    unsafe {
        let hmonitor = MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONULL);
        if hmonitor.is_null() {
            None
        } else {
            let mut mi = MaybeUninit::<MONITORINFO>::uninit();
            (*mi.as_mut_ptr()).cbSize = mem::size_of::<MONITORINFO>() as u32;
            GetMonitorInfoA(hmonitor, mi.as_mut_ptr());
            Some(mi.assume_init())
        }
    }
}

pub fn get_taskbar_height() -> i32 {
    unsafe {
        let hmonitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);
        let mut mi = MaybeUninit::<MONITORINFO>::uninit();
        (*mi.as_mut_ptr()).cbSize = mem::size_of::<MONITORINFO>() as u32;
        GetMonitorInfoA(hmonitor, mi.as_mut_ptr());
        mi.assume_init().rcWork.bottom
    }
}
