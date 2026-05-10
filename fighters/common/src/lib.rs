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

pub mod djc;
pub mod misc;
pub mod opff;
pub mod general_statuses;
pub mod function_hooks;

// for storing what team color the last attacker had. used in a couple different common files
pub static mut LAST_ATTACK_TEAM_COLOR: i32 = 0;

extern "C" fn common_init(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_INIT);
}

extern "C" fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ratio =
            (WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0)
                / WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0));
        VarModule::set_float(
            fighter.battle_object,
            vars::common::instance::JUMP_SPEED_RATIO,
            ratio,
        );
    }
}

pub fn install() {
    djc::install();
    misc::install();
    opff::install();
    general_statuses::install();
    function_hooks::install();

    Agent::new("fighter")
        .on_start(common_init)
        .on_start(fighter_reset)
        .install();
}