# Globe

3d globe

## Notes

Problem:

```sh
$ cargo run
[xcb] Unknown sequence number while processing queue
[xcb] Most likely this is a multi-threaded client and XInitThreads has not been called
[xcb] Aborting, sorry about that.
globe: xcb_io.c:269: poll_for_event: Assertion `!xcb_xlib_threads_sequence_lost' failed.
```

currently using [kiss3d](http://kiss3d.org/) on Xorg

### solution 1

potential Alternatives to kiss3d:

- [piston](https://www.piston.rs/) (on second look doesn't look like what we need)
- [ash](https://crates.io/crates/ash)
- [wgpu](https://crates.io/crates/wgpu)

I dont think this will fix the problem, but maybe it will be a general improvement?

### solution 2

don't use Xorg (lol)

- [wayland](https://wayland.freedesktop.org/)

### solution 3

wait till the bug has been fixed or sth
