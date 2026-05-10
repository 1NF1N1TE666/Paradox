#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod waveclone;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
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
    let popo = &mut Agent::new("popo");
    let nana = &mut Agent::new("nana");

    // acmd::install(popo);
    // acmd::install(nana);

    opff::install_popo(popo);
    opff::install_nana(nana);

    popo.install();
    nana.install();

    // status::install();

    // smashline::clone_weapon("cloud", *WEAPON_KIND_CLOUD_WAVE, "popo", "waveclone", true);
}