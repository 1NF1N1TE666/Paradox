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

unsafe fn var_resets(boma: &mut BattleObjectModuleAccessor) {
    let death_statuses = &[*FIGHTER_STATUS_KIND_DEAD,
                                        *FIGHTER_STATUS_KIND_REBIRTH,
                                        *FIGHTER_STATUS_KIND_WIN,
                                        *FIGHTER_STATUS_KIND_LOSE,
                                        *FIGHTER_STATUS_KIND_ENTRY];

    let damage_statuses = &[*FIGHTER_STATUS_KIND_DAMAGE,
                                        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                                        *FIGHTER_STATUS_KIND_DAMAGE_FALL];

    if !boma.is_situation(*SITUATION_KIND_AIR)
    || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT)
    || boma.is_status_one_of(death_statuses) 
    || boma.is_status_one_of(damage_statuses)
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])
    {
        boma.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_COUNT);
    }

    if boma.is_status_one_of(death_statuses) {
        VarModule::off_flag(boma.object(), vars::common::instance::BURST_LIMIT);
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    var_resets(boma);
}
