#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unexpected_cfgs)]
#![allow(static_mut_refs)]
#![feature(proc_macro_hygiene)]

extern crate dynamic;
extern crate locks;
extern crate once_cell;
extern crate rand;
extern crate skyline;
#[cfg(feature = "skyline-web")]
extern crate skyline_web;
extern crate smash;
extern crate smash_rs;
extern crate smash_arc;
extern crate ninput;
extern crate toml;

mod fighters;

use skyline::libc::c_char;
#[cfg(feature = "main_nro")]
use skyline_web::*;
use std::{fs, path::Path};

#[cfg(not(feature = "main_nro"))]
#[unsafe(no_mangle)]
pub fn smashline_install() {
    fighters::install();
}

use skyline::hooks::InlineCtx;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;

#[skyline::hook(offset = 0x235cad0, inline)]
unsafe fn main_menu_quick(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const skyline::hooks::InlineCtx as *mut u8).add(0x300);
    *(sp.add(0x60) as *mut u64) = 0x1100000000;
    let mut slice = std::slice::from_raw_parts_mut(sp.add(0x68), 18);
    slice.copy_from_slice(b"MenuSequenceScene\0");
    let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53040f0)
        as *const u64;
    println!("{:#x}", *mode);
}

#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = ctx.registers[0].x() as *mut u64;
    let ptr2 = ctx.registers[1].x() as *mut u64;

    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

#[skyline::main(name = "hdr")]
pub fn main() {
    #[cfg(feature = "main_nro")]
    {
        skyline::install_hooks!(
            main_menu_quick,
            fix_chara_replace,
        );
    }

    #[cfg(not(feature = "runtime"))]
    {
        utils::init();
    }

    fighters::install();
}