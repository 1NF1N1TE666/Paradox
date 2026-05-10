use super::*;
use globals::*;

//=================================================================
//== WorkModule::is_enable_transition_term
//== Note: Disable transition terms
//==        - Airdodge out of tumble
//==        - Airdodge out of footstool during footstool lockout
//=================================================================
#[skyline::hook(replace=WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if boma.is_fighter() {
        let fighter_kind = boma.kind();
        let status_kind = StatusModule::status_kind(boma);
        let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW].contains(&flag)
            && [*FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status_kind) {
            return true;
        }

        if fighter_kind == *FIGHTER_KIND_NANA {
            if ([*FIGHTER_STATUS_KIND_WAIT, 
                *FIGHTER_STATUS_KIND_TURN, 
                *FIGHTER_STATUS_KIND_WALK, 
                *FIGHTER_STATUS_KIND_WALK_BRAKE, 
                *FIGHTER_STATUS_KIND_RUN_BRAKE, 
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_WAIT,
                *FIGHTER_STATUS_KIND_SQUAT_RV].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) || 
            ([*FIGHTER_STATUS_KIND_DASH, 
                *FIGHTER_STATUS_KIND_TURN_DASH, 
                *FIGHTER_STATUS_KIND_RUN].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) || 
            ([*FIGHTER_STATUS_KIND_DASH,
                *FIGHTER_STATUS_KIND_TURN_DASH, 
                *FIGHTER_STATUS_KIND_RUN].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
                return true;
            }
        }
    }

    original!()(boma, flag)
}

#[skyline::hook(replace=WorkModule::enable_transition_term)]
unsafe fn enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN {
        VarModule::on_flag(boma.object(), vars::common::status::IS_DASH_TO_RUN_FRAME);
    }
    original!()(boma, flag)
}

pub fn install() {
    skyline::install_hooks!(
        is_enable_transition_term_hook,
        enable_transition_term_hook,
    );
}