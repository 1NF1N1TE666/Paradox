use super::*;
use globals::*;

unsafe fn cancel_frames(boma: &mut BattleObjectModuleAccessor) {
    if (boma.is_motion(Hash40::new("landing_fall_special")) && MotionModule::frame(boma) >= 1.0)
    || (boma.is_motion(Hash40::new("attack_100_end")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("attack_dash")) && MotionModule::frame(boma) >= 21.0)
    || (boma.is_motion(Hash40::new("attack_s3_s")) && MotionModule::frame(boma) >= 21.0)
    || (boma.is_motion(Hash40::new("attack_s3_s2")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("attack_s3_s3")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("attack_hi3")) && MotionModule::frame(boma) >= 23.0)
    || (boma.is_motion(Hash40::new("attack_lw3")) && MotionModule::frame(boma) >= 11.0)
    || (boma.is_motion(Hash40::new("attack_s4")) && MotionModule::frame(boma) >= 40.0)
    || (boma.is_motion(Hash40::new("attack_hi4")) && MotionModule::frame(boma) >= 41.0)
    || (boma.is_motion(Hash40::new("attack_lw4")) && MotionModule::frame(boma) >= 21.0)
    || (boma.is_motion(Hash40::new("attack_air_n")) && MotionModule::frame(boma) >= 39.0)
    || (boma.is_motion(Hash40::new("attack_air_f")) && MotionModule::frame(boma) >= 19.0)
    || (boma.is_motion(Hash40::new("attack_air_b")) && MotionModule::frame(boma) >= 44.0)
    || (boma.is_motion(Hash40::new("attack_air_hi")) && MotionModule::frame(boma) >= 10.0)
    || (boma.is_motion(Hash40::new("attack_air_lw")) && MotionModule::frame(boma) >= 20.0)
    || (boma.is_motion(Hash40::new("catch")) && MotionModule::frame(boma) >= 22.0)
    || (boma.is_motion(Hash40::new("catch_dash")) && MotionModule::frame(boma) >= 24.0)
    || (boma.is_motion(Hash40::new("catch_turn")) && MotionModule::frame(boma) >= 23.0)
    || (boma.is_motion(Hash40::new("throw_f")) && MotionModule::frame(boma) >= 18.0)
    || (boma.is_motion(Hash40::new("throw_b")) && MotionModule::frame(boma) >= 42.0)
    || (boma.is_motion(Hash40::new("throw_hi")) && MotionModule::frame(boma) >= 62.0)
    || (boma.is_motion(Hash40::new("throw_lw")) && MotionModule::frame(boma) >= 81.0) {
        CancelModule::enable_cancel(boma);
    }
}

unsafe fn special_s(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        fighter.on_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT);
        fighter.change_status_req(*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, false);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    cancel_frames(boma);
    special_s(fighter);
}

pub unsafe extern "C" fn metaknight_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, metaknight_frame);
}