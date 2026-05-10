utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn landing_fall_special(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)
    && MotionModule::frame(boma) >= 1.0 {
        CancelModule::enable_cancel(boma);
    }
}

unsafe fn special_s(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH)
    && AttackModule::is_infliction_status(fighter.boma(), *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        fighter.on_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT);
        fighter.change_status_req(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    landing_fall_special(boma);
    special_s(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn metaknight_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        metaknight_frame(fighter);
    }
}

pub unsafe fn metaknight_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, metaknight_frame_wrapper);
}