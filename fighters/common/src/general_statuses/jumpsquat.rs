use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_JumpSquat_Main,
            status_JumpSquat_common,
            uniq_process_JumpSquat_exec_status_param,
            sub_jump_squat_uniq_check_sub,
            sub_jump_squat_uniq_check_sub_mini_attack,
            sub_status_JumpSquat_check_stick_lr_update,
            status_JumpSquat,
            bind_address_call_status_end_JumpSquat,
            status_end_JumpSquat,
        );
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat)]
unsafe fn status_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr_update = fighter.sub_status_JumpSquat_check_stick_lr_update();
    fighter.status_JumpSquat_common(lr_update);
    if fighter.is_button_on(Buttons::Jump | Buttons::FlickJump) & fighter.is_button_on(Buttons::Guard) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_JumpSquat_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe fn status_JumpSquat_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let should_end = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine); callable(fighter).get_bool()} else {false}
    } else { false };
    if should_end {
        return L2CValue::I32(1);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
        fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_ESCAPE_AIR), L2CValue::Bool(true));
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_JUMP), L2CValue::Bool(false));
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_FALL), L2CValue::Bool(false));
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
        fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_CATCH), L2CValue::Bool(true));
        return 0.into();
    }
    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 && fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_SPECIAL_HI), L2CValue::Bool(true));
        } else if !fighter.sub_transition_specialflag_hoist().get_bool() {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[0x58].get_bool() != false && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x58].get_ptr()); callable(fighter).get_bool()} {return L2CValue::I32(0);}
                if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 && fighter.is_situation(*SITUATION_KIND_GROUND) {
                    fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_HI4_START), L2CValue::Bool(true));
                }
            }
        } else {}
    }
    0.into()
}

// end status stuff
#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_JumpSquat)]
unsafe fn bind_address_call_status_end_JumpSquat(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_JumpSquat()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_JumpSquat)]
unsafe fn status_end_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE_SECOND);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_common)]
unsafe fn status_JumpSquat_common(fighter: &mut L2CFighterCommon, lr_update: L2CValue) {
    let is_button_jump = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE) == 0 || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0;
    if is_button_jump {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_BUTTON);
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if lr_update.get_bool() {
        VarModule::on_flag(fighter.battle_object, vars::common::status::CSTICK_IRAR);
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    let potential_enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND
    ];
    for x in potential_enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ABNORMAL_MINIJUMP_SLOWWALK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
    }
    if fighter.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_GUARD_ON, 
        *FIGHTER_STATUS_KIND_GUARD, 
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, 
        *FIGHTER_STATUS_KIND_GUARD_OFF
    ]) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    }
}

#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status_param)]
unsafe fn uniq_process_JumpSquat_exec_status_param(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    if arg.get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(arg.get_ptr());
        callable(fighter);
    } else {
        fighter.sub_jump_squat_uniq_check_sub(L2CValue::I32(*FIGHTER_STATUS_JUMP_FLAG_BUTTON));
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }
    let frame = fighter.global_table[globals::CURRENT_FRAME].get_i32() + 1;
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD)) && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N == 0 {
        if !(fighter.kind() == *FIGHTER_KIND_PICKEL && fighter.is_prev_status_one_of(&[
            *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, 
            *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT
        ])) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
        }
    }
    let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0);
    if frame >= end_frame {
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
        let situation_kind = fighter.global_table[SITUATION_KIND].clone();
        fighter.global_table[PREV_SITUATION_KIND].assign(&situation_kind);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT) && fighter.status() == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if fighter.global_table[STICK_Y].get_f32() <= 0.2 {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            } else {
                VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        } else {
            VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_SQUAT, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
        }
    } else {
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMPSQUAT_VELOCITY, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
        VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
        VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM_SPECIALS, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub)]
unsafe fn sub_jump_squat_uniq_check_sub(fighter: &mut L2CFighterCommon, flag: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    if WorkModule::is_flag(fighter.module_accessor, flag.get_i32()){
        let frame = fighter.global_table[globals::CURRENT_FRAME].get_i32() + 1;
        let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0);
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::is_jump_mini_button(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        if frame >= (end_frame - 1) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE_SECOND);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
            }
            if frame >= (end_frame - 1) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::CSTICK_OVERRIDE);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
            }
        }
    } else {
        if fighter.is_button_on(Buttons::CStickOverride) {
            ControlModule::reset_main_stick_x(fighter.module_accessor);
        }
        if fighter.left_stick_y() < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y")) {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
            || ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        } else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub_mini_attack)]
unsafe fn sub_jump_squat_uniq_check_sub_mini_attack(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON)
            && current_frame > 0.0 {
            FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        }
    } else {
        if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
                return;
            }
        } else if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
            return;
        }
        let unables = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW
        ];
        for x in unables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_JumpSquat_check_stick_lr_update)]
unsafe fn sub_status_JumpSquat_check_stick_lr_update(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    L2CValue::Bool(prev_status == *FIGHTER_STATUS_KIND_DASH && fighter.is_button_on(Buttons::CStickOverride))
}