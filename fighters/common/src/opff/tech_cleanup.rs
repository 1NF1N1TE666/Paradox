use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

//==========================================================================
//== WAVEDASH TURN BUFFER CLEAR
//== Clears turns out of the buffer after a wavedash for smoother movement
//==========================================================================

unsafe fn wavedash_turn_clear(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&StatusModule::prev_status_kind(boma, 0))
        && [*FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING].contains(&StatusModule::status_kind(boma)){
        if boma.is_cat_flag(Cat1::Turn) {
            ControlModule::clear_command(boma, true);
        }

    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {
    wavedash_turn_clear(boma, cat[0], status_kind, situation_kind);
}
