use smash::app::lua_bind::*;
use smash::app::*;
use smash::phx::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use utils::consts::*;
use utils::ext::*;
use utils::*;
use smashline::*;
use smash_script::macros::{EFFECT_FOLLOW, EFFECT_FOLLOW_FLIP, LAST_EFFECT_SET_COLOR};
use globals::*;

pub fn install() {
    smashline::Agent::new("fighter")
        .on_start(init)
        .on_line(Main, opff)
        .on_line(Main, paradox_drop)
        .on_line(Main, paradox_cancel)
        .on_line(Main, paradox_clutch)
        .on_line(Main, paradox_burst)
        .install();
}

pub unsafe extern "C" fn init(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.object(), vars::common::instance::PARADOX_BURST_LIMIT);
}

pub unsafe extern "C" fn opff(fighter: &mut L2CFighterCommon) {
    fighter.check_paradox_dodge();
    fighter.check_hitfall();

    fighter.unable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D);

    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_STANDBY,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL
    ]) || GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) 
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    || lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) 
    || !sv_information::is_ready_go() {
        VarModule::set_int(fighter.object(), vars::common::instance::AIR_TIME, 0);
    } else {
        VarModule::add_int(fighter.object(), vars::common::instance::AIR_TIME, 1);
        if VarModule::get_int(fighter.object(), vars::common::instance::AIR_TIME) == 480 {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }

    if KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs() >= 4.0
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
    ]) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_DEAD, false);
    }
}

pub unsafe extern "C" fn paradox_drop(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD_OFF
    ]) && fighter.is_button_trigger(Buttons::AppealAll) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
    }
}

pub unsafe extern "C" fn paradox_cancel(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        fighter.check_dash_cancel();
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        if !fighter.is_status_one_of(&[
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_F,
        ]) {
            fighter.check_jump_cancel(true);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        fighter.check_airdodge_cancel();
        fighter.check_magicseries();
    }
}

pub unsafe extern "C" fn paradox_clutch(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_DASH, 
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
        *FIGHTER_STATUS_KIND_CATCH_DASH,
        *FIGHTER_STATUS_KIND_CATCH_DASH_PULL,
        *FIGHTER_STATUS_KIND_CATCH_TURN,
        *FIGHTER_STATUS_KIND_CATCH_WAIT,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH,
        *FIGHTER_STATUS_KIND_THROW,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_WALL,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH_TURN,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH_TURN,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH,
        *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_HI_START,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_START,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_APPEAR,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH,
        statuses::ridley::SPECIAL_LW_POGO
    ]) {
        if fighter.is_button_trigger(Buttons::AppealAll) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: -1.0, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }
}

pub unsafe extern "C" fn paradox_burst(fighter: &mut L2CFighterCommon) {
    let lr = fighter.lr();
    let flash_y_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);

    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0
    && !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY])
    && fighter.is_button_trigger(Buttons::AppealAll)
    && !VarModule::is_flag(fighter.object(), vars::common::instance::PARADOX_BURST_LIMIT) {
        VarModule::on_flag(fighter.object(), vars::common::instance::PARADOX_BURST_LIMIT);
        smash::app::FighterUtil::flash_eye_info(fighter.module_accessor);
        if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, flash_y_offset, 2, 0, 0, 0, 0.66, true, *EF_FLIP_YZ);
        } else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, flash_y_offset, 2, 0, 0, 0, 0.66, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.83, 0.69, 0.22);
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}