#![feature(core_intrinsics)]
#![no_main]

use w3_std::{cc, driver::spawn, info};
use w3_sys::native;

extern crate alloc;

// async unsafe fn spawn_unit_no_refcnt_manipulation() {
//     let u = ffi::create_unit(ffi::player(0), cc!("hfoo"), 0., 0., 0.);
//     let u_h = u.engine_id();
//     info!("u: {}", u_h);

//     info!("deleting {u_h}");
//     ffi::remove_unit(u);
//     info!("deleted {u_h}");

//     timer_once(Duration::from_secs(1)).await;

//     let u2 = ffi::create_unit(ffi::player(0), cc!("hfoo"), 0., 0., 0.);
//     let u2_h = u2.engine_id();
//     info!("u2: {}", u2_h);
// }

// async unsafe fn spawn_unit_trefcnt_track() {
//     let u = ffi::create_unit(ffi::player(0), cc!("hfoo"), 0., 0., 0.);
//     let u_h = u.engine_id();
//     info!("u: {}", u_h);

//     __trefcnt_inc(u_h);

//     info!("deleting {u_h}");
//     ffi::remove_unit(u);
//     info!("deleted {u_h}");

//     timer_once(Duration::from_secs(1)).await;

//     let u2 = ffi::create_unit(ffi::player(0), cc!("hfoo"), 0., 0., 0.);
//     let u2_h = u2.engine_id();
//     info!("u2: {}", u2_h);
// }

// async unsafe fn spawn_unit_handleref_track() {
//     let u = ffi::create_unit(ffi::player(0), cc!("hfoo"), 0., 0., 0.);
//     let u_h = u.engine_id();
//     info!("u: {}", u_h);

//     __handle_refcnt_inc(u_h);

//     info!("deleting {u_h}");
//     ffi::remove_unit(u);
//     info!("deleted {u_h}");

//     timer_once(Duration::from_secs(1)).await;

//     let u2 = ffi::create_unit(ffi::player(0), cc!("hfoo"), 0., 0., 0.);
//     let u2_h = u2.engine_id();
//     info!("u2: {}", u2_h);
// }

#[no_mangle]
unsafe extern "C" fn main() {
    spawn(async {
        info!("Hello world from Rust and WASM!");

        let p = native::player(0).unwrap().promote();
        native::create_unit(&p, cc!("hfoo"), 0., 0., 0.)
            .unwrap()
            .promote();

        let p_name = native::get_player_name(&p);
        info!("p: {}", p_name);
    });
}

#[no_mangle]
extern "C" fn config() {}

extern "C" {
    fn __trefcnt_inc(i: i32);
    fn __trefcnt_dec(i: i32);
    fn __trefcnt_get(i: i32) -> i32;

    fn __handle_refcnt_inc(i: i32);
    fn __handle_refcnt_dec(i: i32);
    fn __handle_refcnt_get(i: i32) -> i32;
}
