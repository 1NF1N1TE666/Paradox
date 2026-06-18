utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn cancel_frames(boma: &mut BattleObjectModuleAccessor) {
    if (boma.is_motion(Hash40::new("landing_fall_special")) && MotionModule::frame(boma) >= 1.0)
    || (boma.is_motion(Hash40::new("attack_100_end")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("attack_dash")) && MotionModule::frame(boma) >= 21.0)
    || (boma.is_motion(Hash40::new("attack_s3_s")) && MotionModule::frame(boma) >= 21.0)
    || (boma.is_motion(Hash40::new("attack_s3_s2")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("attack_s3_s3")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("attack_lw3")) && MotionModule::frame(boma) >= 11.0)
    || (boma.is_motion(Hash40::new("attack_s4")) && MotionModule::frame(boma) >= 40.0)
    || (boma.is_motion(Hash40::new("attack_hi4")) && MotionModule::frame(boma) >= 41.0)
    || (boma.is_motion(Hash40::new("attack_lw4")) && MotionModule::frame(boma) >= 21.0)
    || (boma.is_motion(Hash40::new("attack_air_n")) && MotionModule::frame(boma) >= 39.0)
    || (boma.is_motion(Hash40::new("attack_air_f")) && MotionModule::frame(boma) >= 21.0)
    || (boma.is_motion(Hash40::new("attack_air_b")) && MotionModule::frame(boma) >= 44.0)
    || (boma.is_motion(Hash40::new("attack_air_hi")) && MotionModule::frame(boma) >= 10.0)
    || (boma.is_motion(Hash40::new("attack_air_lw")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("throw_f")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("throw_b")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("throw_hi")) && MotionModule::frame(boma) >= 60.0)
    || (boma.is_motion(Hash40::new("throw_lw")) && MotionModule::frame(boma) >= 80.0) {
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

unsafe fn special_lw(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && boma.status_frame() >= 12 && boma.is_button_trigger(Buttons::Special) {
        boma.change_status_req(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    cancel_frames(boma);
    special_s(fighter);
    special_lw(boma);
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