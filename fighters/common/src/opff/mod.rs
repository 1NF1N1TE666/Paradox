use super::*;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;

pub mod ledges;
pub mod physics;
pub mod tech;
pub mod tech_cleanup;
pub mod var_resets;
pub mod momentum_transfer_line;
pub mod gimmick;
pub mod other;
pub mod pocket;

use other::*;

/*
This function runs exactly once per every fighter loaded into a match, every frame. I.E.  5 players in a match = 5 times per frame
Use this instead of get_command_flag_cat
*/

// This is a special case function (I.E. don't use this as an exmaple for hooking).
// It doesn't need a hook or return value because all that is handled in the ACMD crate.
// lol, lmao - blujay

// general per-frame fighter-level hooks
#[utils::export(common::opff)]
pub unsafe fn fighter_common_opff(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        let boma = &mut *info.boma;
        if boma.is_fighter() {
            moveset_edits(fighter, &info);
        }
        
    } else {
        panic!("Could not get the FrameInfo for this fighter! Is this even a fighter?")
    }
}

pub unsafe fn moveset_edits(fighter: &mut L2CFighterCommon, info: &FrameInfo) {
    let boma = &mut *info.boma;

    if WorkModule::get_int(fighter.boma(), *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 999999 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
    }

    physics::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    tech::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing, info.frame);
    tech_cleanup::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing, info.frame);
    ledges::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    var_resets::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    other::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    momentum_transfer_line::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
}

pub fn install() {
    Agent::new("fighter")
        .install();
}
