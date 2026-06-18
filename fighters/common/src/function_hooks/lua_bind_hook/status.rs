use super::*;
use globals::*;

//=================================================================
//== StatusModule::init_settings
//=================================================================
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, mut situation: smash::app::SituationKind, kinetic_type: i32, ground_correct_kind: u32,
                             ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool,
                             keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let mut cliff_check_kind = ground_cliff_check_kind;
    let mut kinetic_type = kinetic_type.clone();
    let mut ground_correct_kind = ground_correct_kind.clone();
                                
    // Call edge_slippoffs init_settings
    ground_correct_kind = super::ground::init_settings_edges(boma, situation, kinetic_type, ground_correct_kind, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);

    if boma.is_fighter() {
        
        if boma.is_prev_situation(*SITUATION_KIND_AIR)
        && ( situation.0 == *SITUATION_KIND_GROUND
            || (boma.is_situation(*SITUATION_KIND_GROUND) && situation.0 == *SITUATION_KIND_NONE) )
        {
            if kinetic_type == *FIGHTER_KINETIC_TYPE_MOTION {
                kinetic_type = *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL;
            }
        }

        // Resets your airtime counter when leaving the below statuses
        // Prevents ECB from shifting on f1 after an "ignored" status (those defined below)
        if boma.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEMO,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_CATCHED_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD])
        && situation.0 == *SITUATION_KIND_AIR
        {
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
        }

        // Walk through other fighters
        JostleModule::set_team(boma, 0);

        if boma.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_SQUAT])
        && boma.is_status(*FIGHTER_STATUS_KIND_LANDING)
        && GroundModule::is_passable_ground(boma) {
            ControlModule::reset_flick_y(boma);
        }

        if ground_correct_kind == *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32 {
            ground_correct_kind = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32;
        }
    }

    // VarModule Status Variable reset checks
    // This makes the assumption that if the KEEP_FLAG is not NONE, you want to clear the
    // status variable array for that data type. Because Smash shares its space between
    // INT and INT64, I have included both of them under a single check.
    let object = boma.object();
    if VarModule::has_var_module(object) {
        let mut mask = 0;
        if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG {
            mask += VarModule::RESET_STATUS_FLAG;
        }
        if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT {
            mask += VarModule::RESET_STATUS_INT;
            mask += VarModule::RESET_STATUS_INT64;
        }
        if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT {
            mask += VarModule::RESET_STATUS_FLOAT;
        }
        VarModule::reset(object, mask);
    }

    original!()(boma, situation, kinetic_type, ground_correct_kind, cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;

    if boma.is_fighter() {
        if [
            *FIGHTER_STATUS_KIND_PASSIVE_WALL,
            *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP,
            *FIGHTER_STATUS_KIND_PASSIVE_CEIL,
            *FIGHTER_STATUS_KIND_CATCH_ATTACK
        ].contains(&next_status) {
            return 0;
        }

        if next_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if boma.is_cat_flag(Cat1::AirEscape) && !boma.is_cat_flag(Cat1::AttackN) {
                VarModule::on_flag(boma.object(), vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
            }
        }

        if boma.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR)
        && next_status == *FIGHTER_STATUS_KIND_LANDING
        && boma.motion_frame() < 1.0 {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_CC_NON_TUMBLE);
        }

        if [
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
        ].contains(&next_status) {
            next_status = *FIGHTER_STATUS_KIND_DOWN;
        }

        if [
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
        ].contains(&next_status) {
            clear_buffer = true;
        }

        VarModule::set_int(boma.object(), vars::common::instance::PREV_STATUS_TRANSITION_FRAME, util::get_fighter_common_from_accessor(boma).global_table[CURRENT_FRAME].get_i32());
        VarModule::set_flag(boma.object(), vars::common::instance::WAS_PREV_STATUS_CANCELABLE, CancelModule::is_enable_cancel(boma) || (boma.is_status(*FIGHTER_STATUS_KIND_LANDING) && WorkModule::is_flag(boma, *FIGHTER_STATUS_LANDING_FLAG_STIFF_CANCEL_CONT)));
    }

    original!()(boma, next_status, clear_buffer)
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_hook,
        change_status_request_from_script_hook
    );
}