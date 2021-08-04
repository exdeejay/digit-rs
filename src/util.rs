use std::mem::{self, MaybeUninit};
use winapi::{
    shared::windef::POINT,
    um::winuser::{
        GetMonitorInfoA, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONULL,
        MONITOR_DEFAULTTOPRIMARY,
    },
};

pub fn get_monitorinfo(x: i32, y: i32) -> Option<MONITORINFO> {
    unsafe {
        let hmonitor = MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONULL);
        if hmonitor.is_null() {
            None
        } else {
            let mut mi = MaybeUninit::<MONITORINFO>::uninit().assume_init();
            mi.cbSize = mem::size_of::<MONITORINFO>() as u32;
            GetMonitorInfoA(hmonitor, &mut mi);
            Some(mi)
        }
    }
}

pub fn get_taskbar_height() -> i32 {
    unsafe {
        let hmonitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);
        let mut mi = MaybeUninit::<MONITORINFO>::uninit().assume_init();
        mi.cbSize = mem::size_of::<MONITORINFO>() as u32;
        GetMonitorInfoA(hmonitor, &mut mi);
        mi.rcWork.bottom
    }
}
