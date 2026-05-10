use super::*;

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER) {
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, Hash40::new("loop"), false, -1.0);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

unsafe extern "C" fn game_specialairnloop(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER) {
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, Hash40::new("loop"), false, -1.0);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

unsafe extern "C" fn game_specialhihold(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 100, 100, 0, 10.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhiholdair(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 100, 100, 0, 10.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 0.5, 367, 100, 100, 0, 10.0, 5.0, -2.5, -1.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 2.5, 90, 250, 0, 0, 10.0, 5.0, -2.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("reflector"), 10.0, 90, 50, 0, 100, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ReflectorModule::set_status(boma, *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		EFFECT_FOLLOW(agent, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2.0, 0, 0, 0, 1, true);
		EffectModule::preset_limit_num(boma, 2);
		EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("falco_ref_ref"), Hash40::new("top"), 0, 7.27, -2.0, 0, 0, 0, 0.75, true);
	}
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_on"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_speciallwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::falco::status::SPECIAL_LW_SET_EFFECT) {
            EFFECT_FOLLOW(agent, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
            VarModule::on_flag(agent.battle_object, vars::falco::status::SPECIAL_LW_SET_EFFECT);
        }
        FLASH(agent, 0, 0.5, 1, 0.25);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
}

unsafe extern "C" fn expression_speciallwloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_animcmd::wait_loop_sync_mot(lua_state);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 15.0);
    ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
}

unsafe extern "C" fn expression_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ReflectorModule::set_status(boma, *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::falco::status::SPECIAL_LW_SET_EFFECT) {
            EFFECT_FOLLOW(agent, Hash40::new("falco_ref_loop"), Hash40::new("top"), 0, 7, -2, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("falco_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
            VarModule::on_flag(agent.battle_object, vars::falco::status::SPECIAL_LW_SET_EFFECT);
        }
    }
}

unsafe extern "C" fn sound_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_falco_special_l03"));
    }
}

unsafe extern "C" fn expression_speciallwhit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_justshield"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnloop", game_specialnloop, Priority::Low);
    agent.acmd("game_specialairnloop", game_specialairnloop, Priority::Low);
    agent.acmd("game_specialhihold", game_specialhihold, Priority::Low);
    agent.acmd("game_specialhiholdair", game_specialhiholdair, Priority::Low);
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);
    agent.acmd("game_speciallwloop", game_speciallwloop, Priority::Low);
    agent.acmd("game_specialairlwloop", game_speciallwloop, Priority::Low);
    agent.acmd("effect_speciallwloop", effect_speciallwloop, Priority::Low);
    agent.acmd("effect_specialairlwloop", effect_speciallwloop, Priority::Low);
    agent.acmd("sound_speciallwloop", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairlwloop", acmd_stub, Priority::Low);
    agent.acmd("expression_speciallwloop", expression_speciallwloop, Priority::Low);
    agent.acmd("expression_specialairlwloop", expression_speciallwloop, Priority::Low);
    agent.acmd("game_speciallwend", acmd_stub, Priority::Low);
    agent.acmd("game_specialairlwend", acmd_stub, Priority::Low);
    agent.acmd("effect_speciallwend", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairlwend", acmd_stub, Priority::Low);
    agent.acmd("sound_speciallwend", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairlwend", acmd_stub, Priority::Low);
    agent.acmd("expression_speciallwend", expression_speciallwend, Priority::Low);
    agent.acmd("expression_specialairlwend", expression_speciallwend, Priority::Low);
    agent.acmd("game_speciallwhit", game_speciallwhit, Priority::Low);
    agent.acmd("game_specialairlwhit", game_speciallwhit, Priority::Low);
    agent.acmd("effect_speciallwhit", effect_speciallwhit, Priority::Low);
    agent.acmd("effect_specialairlwhit", effect_speciallwhit, Priority::Low);
    agent.acmd("sound_speciallwhit", sound_speciallwhit, Priority::Low);
    agent.acmd("sound_specialairlwhit", sound_speciallwhit, Priority::Low);
    agent.acmd("expression_speciallwhit", expression_speciallwhit, Priority::Low);
    agent.acmd("expression_specialairlwhit", expression_speciallwhit, Priority::Low);
}
