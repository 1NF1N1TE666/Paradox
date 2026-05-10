use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;

unsafe fn ledge_act(boma: &mut BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if boma.is_status(*FIGHTER_STATUS_KIND_CLIFF_CATCH) {
        if fighter_kind != *FIGHTER_KIND_NANA {
            if boma.status_frame() > 5 {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
            }
        }
    }
}

unsafe fn global_faf_ledgegrab(fighter: &mut L2CFighterCommon) {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    if fighter.is_situation(*SITUATION_KIND_AIR)
    && FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true) > 0.0
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        GroundModule::set_cliff_check(fighter.module_accessor, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        fighter.sub_transition_group_check_air_cliff();
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    ledge_act(boma, status_kind, fighter_kind);
    global_faf_ledgegrab(fighter);
}