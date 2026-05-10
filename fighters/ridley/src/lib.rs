#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod breath;

use std::f32::consts::PI;
use smash::{
    lib::{
        L2CValue,
        LuaConst,
        *
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait,
            get_value_float
        },
        utility,
        lua_bind::{
            *,
            Article,
            KineticEnergy,
            KineticEnergyNormal,
            FighterKineticEnergyController,
            FighterManager,
            FighterInformation,
            FighterCutInManager
        }    
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::{
        *,
        Hash40,
        Vector2f,
        Vector3f
    },

};
use smash_script::{
    *,
    macros::*
};
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub fn install() {
    smashline::update_weapon_count(*WEAPON_KIND_RIDLEY_BREATH, 10);
    let agent = &mut Agent::new("ridley");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    breath::install();
}