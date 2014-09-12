rust-tinywm
===========

This is a rust port of [TinyWM](http://incise.org/tinywm.html). 

Try it out
==========

::
 Xephyr -ac -br -screen 800x600 :1.0 &
 cargo run & xterm
 
Usage
=========
  Focus follows pointer.
  Alt+Button1, drag: interactive window move
  Alt+Button3, drag: interactive window resize
  Alt+F1: raise focused window
