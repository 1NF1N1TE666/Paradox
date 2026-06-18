use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.7);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn sound_catch(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_06"));
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_swing_06"));
    }
}

unsafe extern "C" fn expression_catch(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.55555);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn sound_catchdash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_06"));
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_swing_06"));
    }
}

unsafe extern "C" fn expression_catchdash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 10.0, -4.0, Some(0.0), Some(10.0), Some(-14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn sound_catchturn(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_06"));
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_swing_06"));
    }
}

unsafe extern "C" fn expression_catchturn(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn expression_catchpull(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn expression_catchwait(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn expression_catchcut(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 361, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 2.0, 361, 100, 0, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 2.0, 361, 100, 0, 0, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 25, 15);
        ATTACK(agent, 0, 0, Hash40::new("footr"), 2.0, 361, 100, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn effect_throwf(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -17, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_throwf(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_swing_m"));
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
}

unsafe extern "C" fn expression_throwf(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, 0); 
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 361, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(agent, 25, 15);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 3.0, z: 0.0});
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn effect_throwb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -3, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_throwb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

unsafe extern "C" fn expression_throwb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 90, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        let weapon = if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE} else {*FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE};
        ArticleModule::generate_article(boma, weapon, false, -1);
        ArticleModule::shoot_exist(boma, weapon, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        ArticleModule::set_visibility_whole(boma, weapon, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("samus_missile_shot"), Hash40::new("haver"), 2.2, 0.4, -0.15, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("samus_missile_straight_smoke"), Hash40::new("armr"), 0, 0, 4, 0, 0, 90, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
}

unsafe extern "C" fn expression_throwhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        ArticleModule::change_motion(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, -1.0);
        ArticleModule::set_rate(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 1.5);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        let link_no = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        MotionModule::set_frame(capture_boma, 1.0, true);
        MotionModule::set_rate(capture_boma, 0.5);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ArticleModule::set_rate(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 1.0);
    }
}

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 270, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {100} else {60}, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {100} else {0}, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {0} else {80}, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_elec_whip"}), *ATTACK_SOUND_LEVEL_S, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_KICK}, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 1, 32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT_FOLLOW(agent, Hash40::new("samus_cshot_hold"), Hash40::new("armr"), 7, -1, 0, 180, 90, 10, 1, true);
            LAST_EFFECT_SET_COLOR(agent,0.25, 1.5, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("armr"), 6, 0, 0, 180, 90, 10, 0.075, true);
        }
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("samus_cshot_hold"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("samus_cshot_shot"), Hash40::new("armr"), 7, -1, 0, 180, 90, 10, 0.2, true);
    }
}

unsafe extern "C" fn sound_throwlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_special_n01"));
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_samus_special_n01"));
        PLAY_SE(agent, Hash40::new("se_samus_special_n03"));
    }
}

unsafe extern "C" fn expression_throwlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
        }
        let storage = ArticleModule::get_float(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        VarModule::set_float(boma.object(), vars::samus::instance::SPECIAL_N_THROW_LW_CHARGE_STORAGE, storage);
        ArticleModule::set_float(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, 0.0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        let link_no = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        LinkModule::set_model_constraint_joint(capture_boma, Hash40::new("trans"));
        MotionModule::change_motion(capture_boma, Hash40::new("down_damage_d3"), 0.0, 1.0, false, 0.0, false, false);
    }
    wait(lua_state, 1.0);
    for _ in 0..35 {
        if is_excute(agent) {
            let charge = ArticleModule::get_float(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
            ArticleModule::set_float(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, charge + 0.015, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 0, false, 0);
        let link_no = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        MotionModule::change_motion(capture_boma, Hash40::new("down_damage_d"), 0.0, 0.0, false, 0.0, false, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        let storage = VarModule::get_float(boma.object(), vars::samus::instance::SPECIAL_N_THROW_LW_CHARGE_STORAGE);
        ArticleModule::set_float(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, storage, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);
    agent.acmd("effect_catch", acmd_stub, Priority::Low);
    agent.acmd("sound_catch", sound_catch, Priority::Low);
    agent.acmd("expression_catch", expression_catch, Priority::Low);
    agent.acmd("game_catchdash", game_catchdash, Priority::Low);
    agent.acmd("effect_catchdash", acmd_stub, Priority::Low);
    agent.acmd("sound_catchdash", sound_catchdash, Priority::Low);
    agent.acmd("expression_catchdash", expression_catchdash, Priority::Low);
    agent.acmd("game_catchturn", game_catchturn, Priority::Low);
    agent.acmd("effect_catchturn", acmd_stub, Priority::Low);
    agent.acmd("sound_catchturn", sound_catchturn, Priority::Low);
    agent.acmd("expression_catchturn", expression_catchturn, Priority::Low);
    agent.acmd("expression_catchpull", expression_catchpull, Priority::Low);
    agent.acmd("expression_catchwait", expression_catchwait, Priority::Low);
    agent.acmd("expression_catchcut", expression_catchcut, Priority::Low);
    agent.acmd("game_catchattack", acmd_stub, Priority::Low);
    agent.acmd("effect_catchattack", acmd_stub, Priority::Low);
    agent.acmd("sound_catchattack", acmd_stub, Priority::Low);
    agent.acmd("expression_catchattack", acmd_stub, Priority::Low);
    agent.acmd("game_throwf", game_throwf, Priority::Low);
    agent.acmd("effect_throwf", effect_throwf, Priority::Low);
    agent.acmd("sound_throwf", sound_throwf, Priority::Low);
    agent.acmd("expression_throwf", expression_throwf, Priority::Low);
    agent.acmd("game_throwb", game_throwb, Priority::Low);
    agent.acmd("effect_throwb", effect_throwb, Priority::Low);
    agent.acmd("sound_throwb", sound_throwb, Priority::Low);
    agent.acmd("expression_throwb", expression_throwb, Priority::Low);
    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
    agent.acmd("effect_throwhi", effect_throwhi, Priority::Low);
    agent.acmd("sound_throwhi", sound_throwhi, Priority::Low);
    agent.acmd("expression_throwhi", expression_throwhi, Priority::Low);
    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
    agent.acmd("effect_throwlw", effect_throwlw, Priority::Low);
    agent.acmd("sound_throwlw", sound_throwlw, Priority::Low);
    agent.acmd("expression_throwlw", expression_throwlw, Priority::Low);
}
