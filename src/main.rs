extern crate libc;
extern crate x11;

use std::ffi::CString;
use std::mem::zeroed;

use libc::{c_int, c_uint};

use x11::xlib;

static GRABMODEASYNC : c_int = 1;

static MOD1MASK : c_uint = (1<<3);
static BUTTONPRESSMASK : c_uint = (1 << 2);
static BUTTONRELEASEMASK : c_uint = (1 << 3);
static POINTERMOTIONMASK : c_uint = (1 << 6);

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    let mut arg0 = 0x0 as i8;
    let display : *mut xlib::Display = unsafe { xlib::XOpenDisplay(&mut arg0) };

    let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };
    let mut start: xlib::XButtonEvent = unsafe { zeroed() };

    if display.is_null() {
        std::process::exit(1);
    }

    let f1 = CString::new("F1").unwrap();
    unsafe {
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, xlib::XStringToKeysym(f1.as_ptr())) as c_int, MOD1MASK,
        xlib::XDefaultRootWindow(display), true as c_int, GRABMODEASYNC, GRABMODEASYNC);

        xlib::XGrabButton(display, 1, MOD1MASK, xlib::XDefaultRootWindow(display), true as c_int,
        BUTTONPRESSMASK|BUTTONRELEASEMASK|POINTERMOTIONMASK, GRABMODEASYNC, GRABMODEASYNC,
        0, 0);
        xlib::XGrabButton(display, 3, MOD1MASK, xlib::XDefaultRootWindow(display), true as c_int,
        BUTTONPRESSMASK|BUTTONRELEASEMASK|POINTERMOTIONMASK, GRABMODEASYNC, GRABMODEASYNC,
        0, 0);
    };

    start.subwindow = 0;

    let mut event: xlib::XEvent = unsafe { zeroed() };

    loop {
        unsafe {
            xlib::XNextEvent(display, &mut event);

            match event.get_type() {
                xlib::KeyPress => {
                    let xkey: xlib::XKeyEvent = From::from(event);
                    if xkey.subwindow != 0 {
                        xlib::XRaiseWindow(display, xkey.subwindow);
                    }
                },
                xlib::ButtonPress => {
                    let xbutton: xlib::XButtonEvent = From::from(event);
                    if xbutton.subwindow != 0 {
                        xlib::XGetWindowAttributes(display, xbutton.subwindow, &mut attr);
                        start = xbutton;
                    }
                },
                xlib::MotionNotify => {
                    if start.subwindow != 0 {
                        let xbutton: xlib::XButtonEvent = From::from(event);
                        let xdiff : c_int = xbutton.x_root - start.x_root;
                        let ydiff : c_int = xbutton.y_root - start.y_root;
                        xlib::XMoveResizeWindow(display, start.subwindow,
                                          attr.x + (if start.button==1 { xdiff } else { 0 }),
                                          attr.y + (if start.button==1 { ydiff } else { 0 }),
                                          max(1, attr.width + (if start.button==3 { xdiff } else { 0 })),
                                          max(1, attr.height + (if start.button==3 { ydiff } else { 0 })));
                    }
                },
                xlib::ButtonRelease => {
                    start.subwindow = 0;
                },
                _ => {}
            };
        }
    }
}
