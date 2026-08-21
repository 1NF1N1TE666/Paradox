use super::*;
use globals::*;

unsafe fn bugfix(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_float(boma.object(), vars::lucario::instance::PREV_SPEED_X) * (KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) * boma.lr()) <= 0.0
    && (VarModule::get_float(boma.object(), vars::lucario::instance::PREV_SPEED_X) - (KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) * boma.lr())).abs() > 0.025
    && VarModule::get_float(boma.object(), vars::lucario::instance::PREV_SPEED_Y) == KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL)
    && boma.lr() == VarModule::get_float(boma.object(), vars::lucario::instance::PREV_LR)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
    ]) {
        KineticModule::mul_speed(boma, &Vector3f::new(0.0, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::add_speed(boma, &Vector3f::new(VarModule::get_float(boma.object(), vars::lucario::instance::PREV_SPEED_X), 0.0, 0.0));
    } else {
        VarModule::set_float(boma.object(), vars::lucario::instance::PREV_SPEED_X, KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) * boma.lr());
        VarModule::set_float(boma.object(), vars::lucario::instance::PREV_SPEED_Y, KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));
        VarModule::set_float(boma.object(), vars::lucario::instance::PREV_LR, boma.lr());
    }
}

unsafe fn aura_maximus(fighter: &mut L2CFighterCommon) {
    if DamageModule::damage(fighter.module_accessor, 0) >= 150.0 {
        VarModule::on_flag(fighter.object(), vars::lucario::instance::AURA_MAXIMUS);
    } else {
        VarModule::off_flag(fighter.object(), vars::lucario::instance::AURA_MAXIMUS);
    }
    if VarModule::is_flag(fighter.object(), vars::lucario::instance::AURA_MAXIMUS) {
        if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)
        && MotionModule::frame(fighter.module_accessor) >= 4.0
        && MotionModule::frame(fighter.module_accessor) <= 80.0 {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.0, 0.0, 0.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
}

unsafe fn special_hi(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END) {
        if fighter.is_cat_flag(Cat1::SpecialN) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, false);
        }
        if fighter.is_cat_flag(Cat1::SpecialS) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if fighter.is_cat_flag(Cat1::SpecialLw) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_cat_flag(Cat1::Catch) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, false);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI3, false);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW3, false);
            }
            if fighter.is_cat_flag(Cat1::AttackN) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK, true);
            }
        } else {
            if fighter.get_aerial() != None {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bugfix(boma);
    aura_maximus(fighter);
    special_hi(fighter);
}

pub unsafe extern "C" fn lucario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, lucario_frame);
}