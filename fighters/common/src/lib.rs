#![deny(deprecated)]
#![allow(unused)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]
#![feature(repr_simd)]
#![feature(simd_ffi)]
use smash::app::lua_bind::*;
use smash::app::utility::*;
use smash::lua2cpp::{*, L2CFighterCommon};
use smash::lib::{*, lua_const::*, L2CAgent};
use smash::phx::*;
use smash::app::*;
use smash::app;
use smash::hash40;
use smash::app::sv_animcmd::*;
use smash::app::sv_math::*;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{InlineCtx, Region, getRegionAddress};
use skyline::libc::*;

#[macro_use] extern crate smash_script;

pub mod opff;
pub mod general_statuses;
pub mod function_hooks;

pub static mut LAST_ATTACK_TEAM_COLOR: i32 = 0;

pub fn install() {
    opff::install();
    general_statuses::install();
    function_hooks::install();
}