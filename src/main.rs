#![feature(globs)]

extern crate libc;
extern crate xlib;

use libc::{c_int, c_uint, c_void};
use xlib::*;
use std::ptr;

//static GrabModeSync : c_int = 0;
static GrabModeAsync : c_int = 1;

static Mod1Mask : c_uint = (1<<3);
static ButtonPressMask : c_uint = (1 << 2);
static ButtonReleaseMask : c_uint = (1 << 3);
static PointerMotionMask : c_uint = (1 << 6);

static KeyPress : c_int = 2;
static ButtonPress : c_int = 4;
static ButtonRelease : c_int = 5;
static MotionNotify : c_int = 6;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    let mut arg0 = 0x0 as i8;
    let mut dpy : *mut Display = unsafe { XOpenDisplay(&mut arg0) };

    let mut attr: XWindowAttributes = unsafe { std::mem::uninitialized() };
    let mut start: XButtonEvent = unsafe { std::mem::uninitialized() };
  
    if dpy.is_null() {
        std::os::set_exit_status(1);
        return;
    }
    
    let mut f1 = "F1".to_c_str();
    unsafe {
        XGrabKey(dpy, XKeysymToKeycode(dpy, XStringToKeysym(f1.as_mut_ptr())) as c_int, Mod1Mask,
            XDefaultRootWindow(dpy), true as c_int, GrabModeAsync, GrabModeAsync);

        XGrabButton(dpy, 1, Mod1Mask, XDefaultRootWindow(dpy), true as c_int, 
                    ButtonPressMask|ButtonReleaseMask|PointerMotionMask, GrabModeAsync, GrabModeAsync,
                    0, 0);
        XGrabButton(dpy, 3, Mod1Mask, XDefaultRootWindow(dpy), true as c_int,
                    ButtonPressMask|ButtonReleaseMask|PointerMotionMask, GrabModeAsync, GrabModeAsync,
                    0, 0);
    };

    println!("Its runnning");
    
    start.subwindow = 0;
    loop {
        unsafe { 
            let mut ev : XEvent = std::mem::uninitialized();
            XNextEvent(dpy, &mut ev);
            let data : *mut c_void  = &mut ev as *mut c_void;
            let ev_type = (&mut *(data as *mut XAnyEvent))._type;
            println!("{}", ev_type);
            match ev_type {
                KeyPress => {
                    let xkey = &mut *(data as *mut XKeyEvent);
                    if xkey.subwindow != 0 {
                        XRaiseWindow(dpy, xkey.subwindow);
                    }
                    return;
                },
                ButtonPress => {
                    let xbutton = &mut *(data as *mut XButtonEvent);
                    if xbutton.subwindow != 0 {
                        XGetWindowAttributes(dpy, xbutton.subwindow, &mut attr);
                        start = *xbutton;
                    }
                    return;
                },
                MotionNotify => {
                    if start.subwindow != 0 {
                        let xbutton = &mut *(data as *mut XButtonEvent);
                        let xdiff : c_int = xbutton.x_root - start.x_root;
                        let ydiff : c_int = xbutton.y_root - start.y_root;
                        XMoveResizeWindow(dpy, start.subwindow,
                                          attr.x + (if start.button==1 { xdiff } else { 0 }),
                                          attr.y + (if start.button==1 { ydiff } else { 0 }),
                                          max(1, attr.width + (if start.button==3 { xdiff } else { 0 })),
                                          max(1, attr.height + (if start.button==3 { ydiff } else { 0 })));
                    }
                    return;
                },
                ButtonRelease => {
                    start.subwindow = 0;
                },
                _ => {}
            };
        }
        println!("Event happened!");
    }
}
