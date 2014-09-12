#![feature(globs)]

extern crate libc;
extern crate xlib;

use libc::{c_int, c_uint, c_void};
use xlib::*;

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

fn get_type(data: *mut c_void) -> c_int { 
    unsafe { let ev = &mut *(data as *mut XAnyEvent); return ev._type; };
}

fn cast_event<T>( data: *mut c_void) -> T { 
    unsafe { *(data as *mut T) }
}

fn main() {
    let arg0 : *mut i8 = 0x0 as *mut i8;
    let dpy : *mut Display = unsafe { XOpenDisplay(arg0) };

  //  let attr: XWindowAttributes;
    let mut start: XButtonEvent;
  
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
    
    start.subwindow = 0;
    loop {
        unsafe { 
            let mut ev : XEvent = std::mem::uninitialized();
            XNextEvent(dpy, &mut ev);
            let ev_type = get_type(&mut ev);
            match ev_type {
                KeyPress => { 
                    let key_ev = cast_event::<XKeyEvent>(&mut ev);
                    //XRaiseWindow(dpy, key_ev.subwindow);
                    return;
                },
                ButtonPress => { return; },
                MotionNotify => { return; },
                ButtonRelease => { return; },
                _ => {}
            };
        }
    }
}
