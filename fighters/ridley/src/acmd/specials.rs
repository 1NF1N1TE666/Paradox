use super::*;

unsafe extern "C" fn game_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.11111);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 361, 140, 0, 0, 10.0, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_bleath_hold"), Hash40::new("mouth1"), 5, 1, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_bleath_hold"), false, false);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(agent, Hash40::new("vc_ridley_special_s02"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_smash_s02"));
    }
}

unsafe extern "C" fn expression_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

unsafe extern "C" fn effect_specialairnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_specialsdrag(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 100, 100, 0, 100, 7.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_DRAG_MOVE);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 2.0, 100, 100, 100, 100, 7.0, 0.0, 1.0, 2.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
}

unsafe extern "C" fn game_specialsdragf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("throw"), 2.0, 100, 100, 100, 100, 7.0, 0.0, 1.0, 2.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(boma, true, false);
    }
}

unsafe extern "C" fn game_specialsdragcliff(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 40, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 40, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 35, 17);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 3.0, z: 0.0});
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_REVERT_DEGREE);
    }
}

unsafe extern "C" fn game_specialairsdragjump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 100, 100, 100, 0, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 100, 100, 100, 0, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 30, 5);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: -30.0, y: 0.0, z: 0.0});
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_JUMP);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn game_specialsdragwall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 40, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 40, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_WALL_CROWN_EFFECT);
        FT_CATCH_STOP(agent, 12, 1);
        CHECK_FINISH_CAMERA(agent, 13, 2);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_REVERT_DEGREE);
    }
}

unsafe extern "C" fn game_specialairsfalljump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 100, 100, 100, 0, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 100, 100, 100, 0, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 30, 7);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 50.0, z: 0.0});
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_JUMP);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn game_specialairhichargehi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 24.0, 363, 100, 0, 0, 5.0, 0.0, 10.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("rot"), 10.0, 363, 100, 0, 0, 10.0, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallwstab(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma: &mut BattleObjectModuleAccessor = agent.boma();
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let stick_y = VarModule::get_float(agent.battle_object, vars::ridley::status::SPECIAL_LW_STICK_Y);
        let z_mod = if stick_y > 0.0 {3.0 * stick_y} else if stick_y < 0.0 {-1.0 * stick_y} else {0.0};
        if stick_y > 0.0 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 69.0, 361, 100, 0, 0, 2.0, 0.0, (stick_y * 7.0) + 8.0, 25.0 - z_mod, Some(0.0), Some((stick_y * 11.0) + 7.0), Some(30.0 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 1, 0, Hash40::new("top"), 69.0, 361, 100, 0, 0, 2.0, 0.0, (stick_y * 7.0) + 8.0, 25.0 - z_mod, Some(0.0), Some((stick_y * 11.0) + 7.0), Some(30.0 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 100, 0, 0, 2.0, 0.0, 7.0, 8.0, Some(0.0), Some((stick_y * 11.0) + 7.0), Some(29.5 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        } else if stick_y < 0.0 {
            ATTACK(agent, 0, 0, Hash40::new("top"), 69.0, 361, 100, 0, 0, 2.0, 0.0, (stick_y * 5.0) + 7.0, 25.0 - z_mod, Some(0.0), Some((stick_y * 6.0) + 7.0), Some(30.0 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 1, 0, Hash40::new("top"), 69.0, 361, 100, 0, 0, 2.0, 0.0, (stick_y * 5.0) + 7.0, 25.0 - z_mod, Some(0.0), Some((stick_y * 6.0) + 7.0), Some(30.0 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 100, 0, 0, 2.0, 0.0, 7.0, 8.0, Some(0.0), Some((stick_y * 6.0) + 7.0), Some(29.5 - z_mod), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 69.0, 361, 100, 0, 0, 2.0, 0.0, 7.0, 25.0, Some(0.0), Some(7.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 1, 0, Hash40::new("top"), 69.0, 361, 100, 0, 0, 2.0, 0.0, 7.0, 25.0, Some(0.0), Some(7.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 100, 0, 0, 2.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
        if WorkModule::get_int(boma, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) <= 0 {
            AttackModule::set_no_dead_all(boma, true, false);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwstab(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        let stick_y = VarModule::get_float(agent.battle_object, vars::ridley::status::SPECIAL_LW_STICK_Y);
        let y_mod = if stick_y == 0.0 {0.0} else if stick_y > 0.0 {6.0} else {-3.0};
        let rot = if stick_y > 0.0 {0 - ((stick_y * 25.0) as i32)} else if stick_y < 0.0 {0 - ((stick_y * 15.0) as i32)} else {0};
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, 8.0 + y_mod, 20, rot, 0, 0, 1.0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

unsafe extern "C" fn effect_specialairlwstab(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        let stick_y = VarModule::get_float(agent.battle_object, vars::ridley::status::SPECIAL_LW_STICK_Y);
        let y_mod = if stick_y == 0.0 {0.0} else if stick_y > 0.0 {6.0} else {-3.0};
        let rot = if stick_y > 0.0 {0 - ((stick_y * 25.0) as i32)} else if stick_y < 0.0 {0 - ((stick_y * 15.0) as i32)} else {0};
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, 8.0 + y_mod, 20, rot, 0, 0, 1.0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

unsafe extern "C" fn game_speciallwfinish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 361, 100, 100, 0, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        JostleModule::set_status(boma, true);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn game_specialairlwfinish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 361, 100, 100, 0, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        AttackModule::set_no_finish_camera(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        JostleModule::set_status(boma, true);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn game_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(agent.object(), vars::ridley::status::SPECIAL_LW_POGO_ENABLE_LANDING)
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_BOUNCE)
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("tail8"), 20.0, 300, 100, 0, 0, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("tail8"), 20.0, 300, 100, 0, 0, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 3, 0, Hash40::new("tail7"), 10.0, 361, 100, 0, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(agent, 4, 0, Hash40::new("tail5"), 10.0, 361, 100, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(agent, 5, 0, Hash40::new("tail3"), 10.0, 361, 100, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("tail8"), 10.0, 361, 100, 0, 0, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("tail8"), 10.0, 361, 100, 0, 0, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_BOUNCE);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.object(), vars::ridley::status::SPECIAL_LW_POGO_ENABLE_LANDING)
    }
}

unsafe extern "C" fn effect_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

unsafe extern "C" fn sound_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_jump01"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_special_l01"));
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_special_l02"));
    }
}

unsafe extern "C" fn expression_speciallwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
}

unsafe extern "C" fn effect_specialairlwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

unsafe extern "C" fn sound_specialairlwpogo(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_special_l01"));
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_special_l02"));
    }
}

unsafe extern "C" fn effect_speciallwlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
        EffectModule::set_visible_kind(boma, Hash40::new("ridley_death_stab_line"), false);
        EffectModule::set_visible_kind(boma, Hash40::new("sys_sp_flash"), false);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), -18, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallwlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_landing03"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ridley_attackair_f01"));
    }
}

unsafe extern "C" fn expression_speciallwlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnattack", game_specialnattack, Priority::Low);
    agent.acmd("effect_specialnattack", effect_specialnattack, Priority::Low);
    agent.acmd("sound_specialnattack", sound_specialnattack, Priority::Low);
    agent.acmd("expression_specialnattack", expression_specialnattack, Priority::Low);
    agent.acmd("game_specialairnattack", game_specialnattack, Priority::Low);
    agent.acmd("effect_specialairnattack", effect_specialairnattack, Priority::Low);
    agent.acmd("sound_specialairnattack", sound_specialnattack, Priority::Low);
    agent.acmd("expression_specialairnattack", expression_specialnattack, Priority::Low);
    agent.acmd("game_specialsdrag", game_specialsdrag, Priority::Low);
    agent.acmd("game_specialsdragf", game_specialsdragf, Priority::Low);
    agent.acmd("game_specialsdragcliff", game_specialsdragcliff, Priority::Low);
    agent.acmd("game_specialairsdragjump", game_specialairsdragjump, Priority::Low);
    agent.acmd("game_specialsdragwall", game_specialsdragwall, Priority::Low);
    agent.acmd("game_specialairsfalljump", game_specialairsfalljump, Priority::Low);
    agent.acmd("game_specialairhichargehi", game_specialairhichargehi, Priority::Low);
    agent.acmd("game_speciallwstab", game_speciallwstab, Priority::Low);
    agent.acmd("game_specialairlwstab", game_speciallwstab, Priority::Low);
    agent.acmd("effect_speciallwstab", effect_speciallwstab, Priority::Low);
    agent.acmd("effect_specialairlwstab", effect_specialairlwstab, Priority::Low);
    agent.acmd("game_speciallwfinish", game_speciallwfinish, Priority::Low);
    agent.acmd("game_specialairlwfinish", game_specialairlwfinish, Priority::Low);
    agent.acmd("game_speciallwpogo", game_speciallwpogo, Priority::Low);
    agent.acmd("effect_speciallwpogo", effect_speciallwpogo, Priority::Low);
    agent.acmd("sound_speciallwpogo", sound_speciallwpogo, Priority::Low);
    agent.acmd("expression_speciallwpogo", expression_speciallwpogo, Priority::Low);
    agent.acmd("game_specialairlwpogo", game_speciallwpogo, Priority::Low);
    agent.acmd("effect_specialairlwpogo", effect_specialairlwpogo, Priority::Low);
    agent.acmd("sound_specialairlwpogo", sound_specialairlwpogo, Priority::Low);
    agent.acmd("expression_specialairlwpogo", expression_speciallwpogo, Priority::Low);
    agent.acmd("game_speciallwlanding", acmd_stub, Priority::Low);
    agent.acmd("effect_speciallwlanding", effect_speciallwlanding, Priority::Low);
    agent.acmd("sound_speciallwlanding", sound_speciallwlanding, Priority::Low);
    agent.acmd("expression_speciallwlanding", expression_speciallwlanding, Priority::Low);
}