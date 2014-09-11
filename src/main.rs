#![feature(globs)]

extern crate xlib;

fn main() {
    let mut arg0 : *mut i8 = 0x0 as *mut i8;
    let mut dpy : *mut xlib::Display = unsafe { xlib::XOpenDisplay(arg0) };

    let attr: xlib::XWindowAttributes;
    let start: xlib::XButtonEvent;
    let ev : xlib::XEvent;
    
    if dpy.is_null() {
        std::os::set_exit_status(1);
        return;
    }

    xlib::XGrabKey(dpy, xlib::XKeysymToKeycode(dpy, xlib::XStringToKeysym("F1")), xlib::Mod1Mask,
        xlib::XDefaultRootWindow(dpy), true, xlib::GrabModeAsync, xlib::GrabeModeAsync);
    std::os::set_exit_status(0);
}
