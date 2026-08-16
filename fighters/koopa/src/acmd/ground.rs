use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.15);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 120, 0, 0, 4.0, 0.0, 12.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 120, 0, 0, 4.0, 0.0, 10.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 120, 0, 0, 4.0, 0.0, 8.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.88888);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 8.0, 361, 120, 0, 0, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0, 361, 120, 0, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 8.0, 361, 120, 0, 0, 4.0, -8.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 16.0, 80, 120, 0, 0, 8.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 16.0, 80, 120, 0, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 80, 120, 0, 0, 8.0, 0.0, 24.0, 2.0, Some(0.0), Some(16.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 13.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), -2, 20, 5.5, 19, -8, -52, 1.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EffectModule::set_visible_kind(boma, Hash40::new("koopa_scratch"), false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 13, -20, 0, 0, 0, 2.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.75);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EffectModule::set_visible_kind(boma, Hash40::new("koopa_scratch"), true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if boma.lr() > 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 18.0, 2.0, 0.0, 0,0,140,0.9, 0, 0, 0, 0, 0, 0, false);
        } else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 18.0, 2.0, 0.0, 0,0,-140,0.9, 0, 0, 0, 0, 0, 0, false);
        }
        LAST_EFFECT_SET_RATE(agent,1.5);
    }
}

unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_koopa_attack07"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_smash_l02"));
    }
}

unsafe extern "C" fn expression_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11, Priority::Low);
    agent.acmd("game_attack12", game_attack12, Priority::Low);
    agent.acmd("game_attackdash", game_attackdash, Priority::Low);
    agent.acmd("effect_attackdash", effect_attackdash, Priority::Low);
    agent.acmd("sound_attackdash", sound_attackdash, Priority::Low);
    agent.acmd("expression_attackdash", expression_attackdash, Priority::Low);
}