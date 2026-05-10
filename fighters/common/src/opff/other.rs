use utils::{*, ext::*, consts::*};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd, *};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::phx::*;
use smash::hash40;
use smash::app::sv_animcmd::*;
use smash::app::sv_battle_object;
use smash::app::sv_math;
use smash::app::Fighter;
use smash::app::smashball::*;
use smash_script::*;
use globals::*;
use crate::util::get_fighter_common_from_accessor;

pub unsafe fn cliff_xlu_frame_counter(fighter: &mut L2CFighterCommon) {
    let cliff_xlu_frame = VarModule::get_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
    if cliff_xlu_frame > 0 {
        VarModule::dec_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
        if cliff_xlu_frame - 1 == 0 
        || fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
            VarModule::set_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME, 0);
        }
    }
}

unsafe fn custom_dash_anim_support(fighter: &mut L2CFighterCommon) {
    let run_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::RUN_HIP_OFFSET_X);
    if run_hip_offset_x == 0.0 {
        return;
    }
    
    if fighter.is_status(*FIGHTER_STATUS_KIND_RUN) && fighter.is_motion(Hash40::new("run")) {
        let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::WEIRD_ASS_TURN_RUN_ANIMATION) {
            hip_translate.z -= dash_hip_offset_x - run_hip_offset_x;
        }
        else {
            hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
        }
        
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    }
    else if fighter.is_prev_status(*FIGHTER_STATUS_KIND_RUN)
    && StatusModule::is_changing(fighter.module_accessor)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_TURN_RUN)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_RUN_BRAKE) {
        ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("hip"));
    }
    
    if fighter.is_status(*FIGHTER_STATUS_KIND_TURN_RUN) && fighter.is_motion(Hash40::new("turn_run")) {
        let dash_hip_offset_x = VarModule::get_float(fighter.battle_object, vars::common::instance::DASH_HIP_OFFSET_X);
        let mut hip_translate = Vector3f::zero();
        MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("hip"), false, &mut hip_translate);
        hip_translate.z += dash_hip_offset_x - run_hip_offset_x;
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("hip"), &Vector3f{ x: hip_translate.x, y: hip_translate.y, z: hip_translate.z }, false, false);
    }
    else if fighter.is_prev_status(*FIGHTER_STATUS_KIND_TURN_RUN)
    && StatusModule::is_changing(fighter.module_accessor)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_RUN)
    && !fighter.is_status(*FIGHTER_STATUS_KIND_TURN_RUN_BRAKE) {
        ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("hip"));
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    cliff_xlu_frame_counter(fighter);
    custom_dash_anim_support(fighter);
}