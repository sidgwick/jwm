use std::ffi::CString;
use std::ptr;
use x11::xlib::{
    Display, Screen, Visual, XCreateSimpleWindow, XDefaultColormap, XDefaultDepth, XDefaultGC,
    XDefaultScreen, XDefaultVisual, XDisplayHeight, XDisplayWidth, XGrabServer, XInternAtom,
    XMaxCmapsOfScreen, XOpenDisplay, XRootWindow, XScreenOfDisplay, XSetGraphicsExposures, _XGC,
};

pub fn x_open_display() -> Option<*mut Display> {
    unsafe {
        let res = XOpenDisplay(ptr::null());
        if res.is_null() {
            return None;
        }

        Some(res)
    }
}

pub fn x_default_screen(display: *mut Display) -> i32 {
    unsafe { XDefaultScreen(display) }
}

pub fn x_root_window(display: *mut Display, screen: i32) -> u64 {
    unsafe { XRootWindow(display, screen) }
}

pub fn x_display_width(display: *mut Display, screen: i32) -> i32 {
    unsafe { XDisplayWidth(display, screen) }
}

pub fn x_display_height(display: *mut Display, screen: i32) -> i32 {
    unsafe { XDisplayHeight(display, screen) }
}

pub fn x_default_depth(display: *mut Display, screen: i32) -> i32 {
    unsafe { XDefaultDepth(display, screen) }
}

pub fn x_default_visual(display: *mut Display, screen: i32) -> Option<*mut Visual> {
    unsafe {
        let visual = XDefaultVisual(display, screen);

        if visual.is_null() {
            return None;
        }

        Some(visual)
    }
}

pub fn x_default_colormap(display: *mut Display, screen: i32) -> u64 {
    unsafe { XDefaultColormap(display, screen) }
}

pub fn x_default_gc(display: *mut Display, screen: i32) -> Option<*mut _XGC> {
    unsafe {
        let gc = XDefaultGC(display, screen);
        if gc.is_null() {
            return None;
        }

        Some(gc)
    }
}

pub fn x_max_cmaps_of_screen(screen: *mut Screen) -> i32 {
    unsafe { XMaxCmapsOfScreen(screen) }
}

pub fn x_screen_of_display(display: *mut Display, screen: i32) -> Option<*mut Screen> {
    unsafe {
        let screen = XScreenOfDisplay(display, screen);
        if screen.is_null() {
            return None;
        }

        Some(screen)
    }
}

pub fn x_set_graphics_exposures(
    display: *mut Display,
    gc: *mut _XGC,
    graphics_exposures: bool,
) -> i32 {
    let graphics_exposures = i32::from(graphics_exposures);

    unsafe { XSetGraphicsExposures(display, gc, graphics_exposures) }
}

pub fn x_create_simple_window(
    display: *mut Display,
    parent: u64,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    border_width: u32,
    border: u64,
    background: u64,
) -> u64 {
    unsafe {
        XCreateSimpleWindow(
            display,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            border,
            background,
        )
    }
}

pub fn x_grab_server(display: *mut Display) -> i32 {
    unsafe { XGrabServer(display) }
}

pub fn x_intern_atom(display: *mut Display, name: &String, x: bool) -> u64 {
    let x = i32::from(x);
    let name = CString::from(name);

    unsafe { XInternAtom(display, name.as_ptr(), x) }
}
