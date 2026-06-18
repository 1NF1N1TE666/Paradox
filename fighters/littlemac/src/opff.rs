utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe fn quote_on_quote_air(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_LADDER_ATTACK);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);

    if boma.is_situation(*SITUATION_KIND_GROUND) {
        WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }

    if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) <= 0.0
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE, 
        *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
    ]) {
        boma.change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
    }

    if boma.stick_y() >= 0.0
    && boma.is_button_on(Buttons::Jump) 
    && (boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_WALK_BRAKE,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
    ]) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL)) {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        PostureModule::add_pos_2d(boma, &Vector2f{ x: 0.0, y: 0.75 });
    }

    if boma.stick_y() <= -0.0
    && boma.is_button_on(Buttons::Jump)
    && (boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_B,
        *FIGHTER_STATUS_KIND_SQUAT_F,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_WALK_BRAKE,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
    ]) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL)) {
        PostureModule::add_pos_2d(boma, &Vector2f{ x: 0.0, y: -0.75 });
    }

    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
        WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    }
}

unsafe fn meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let meter: f32 = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if meter < 0.0 {
        crate::vtable_hook::update_littlemac_ui(entry_id, 0.0);
        WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }

    if app::smashball::is_training_mode()
    && boma.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && boma.is_button_trigger(Buttons::Guard) {
        let meter_inc = (meter + 100.0).clamp(0.0, 100.0);
        crate::vtable_hook::update_littlemac_ui(entry_id, meter_inc);
        WorkModule::set_float(boma, meter_inc, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    quote_on_quote_air(boma);
    meter(fighter, boma);
}

pub extern "C" fn littlemac_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        littlemac_frame(fighter)
    }
}

pub unsafe fn littlemac_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, littlemac_frame_wrapper);
}