utils::import_noreturn!(common::opff::fighter_common_opff);
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

unsafe fn laser_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT {
        boma.check_land_cancel(None);
    }
}

unsafe fn special_lw(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        fighter.change_status_req(*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_APPEAR, false);
    }
    
    if fighter.is_status_one_of(&[*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_APPEAR, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_END]) {
        if !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::AURA_MAXIMUS) {
            VarModule::on_flag(fighter.battle_object, vars::lucario::instance::AURA_MAXIMUS);
            VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_DAMAGE_STORAGE, DamageModule::damage(fighter.module_accessor, 0));
            DamageModule::add_damage(fighter.module_accessor, 999.9 - DamageModule::damage(fighter.module_accessor, 0), 0);
        }

        if VarModule::is_flag(fighter.object(), vars::lucario::instance::CANCEL_SPECIAL_LW) {
            if fighter.is_cat_flag(Cat1::SpecialN) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_S, false);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_HI, false);
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
    } else {
        VarModule::off_flag(fighter.battle_object, vars::lucario::instance::CANCEL_SPECIAL_LW);

        if !fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_CATCH,
            *FIGHTER_STATUS_KIND_CATCH_PULL,
            *FIGHTER_STATUS_KIND_CATCH_WAIT,
            *FIGHTER_STATUS_KIND_THROW,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END
        ]) {
            if VarModule::is_flag(fighter.object(), vars::lucario::instance::AURA_MAXIMUS) {
                VarModule::off_flag(fighter.object(), vars::lucario::instance::AURA_MAXIMUS);
                DamageModule::heal(fighter.module_accessor, -DamageModule::damage(fighter.module_accessor, 0), 0);
                if VarModule::get_float(fighter.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE) > 0.0 {
                    DamageModule::add_damage(fighter.module_accessor, VarModule::get_float(fighter.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE), 0);
                    VarModule::set_float(fighter.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE, 0.0);
                }
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW
    ]) {
        fighter.sub_air_check_dive();
    }
}

pub extern "C" fn lucario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        lucario_frame(fighter)
    }
}

pub unsafe fn lucario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bugfix(boma);
    laser_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);
    special_lw(fighter);
    fastfall_specials(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, lucario_frame_wrapper);
}