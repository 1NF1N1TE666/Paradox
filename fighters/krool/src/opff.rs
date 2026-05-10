utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn belly(boma: &mut BattleObjectModuleAccessor) {
    if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON) {
        VarModule::on_flag(boma.object(), vars::krool::instance::WAIST_SHIELD);
    }
    if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF) {
        VarModule::off_flag(boma.object(), vars::krool::instance::WAIST_SHIELD);
    }
}

unsafe fn laser_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW,
    ].contains(&status_kind) {
        boma.check_land_cancel(None);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SUCTION,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SWALLOW,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_CATCH,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_FAILURE,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END,
    ]) && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    belly(boma);
    laser_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);
    fastfall_specials(fighter);
}

pub extern "C" fn krool_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        krool_frame(fighter);
    }
}

pub unsafe fn krool_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, krool_frame_wrapper);
}