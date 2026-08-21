use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 5.0);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(4.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(4.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if ArticleModule::is_generatable(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
            ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0);
        }
    }
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 5.0);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(4.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 8.0, 0.0, 4.0, 10.0, Some(0.0), Some(4.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if ArticleModule::is_generatable(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
            ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0);
        }
    }
}

unsafe extern "C" fn game_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 16.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    FT_MOTION_RATE(agent, 0.535);
    frame(lua_state, 28.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
        CHECK_FINISH_CAMERA(agent, 15, 0);
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn effect_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_near"), Hash40::new("top"), -2, 6, 10, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_SCALE_W(agent, WorkModule::get_float(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLOAT_HAKKEI_NEAR_SCALE), WorkModule::get_float(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLOAT_HAKKEI_NEAR_SCALE), WorkModule::get_float(boma, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLOAT_HAKKEI_NEAR_SCALE));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 27.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }
}

unsafe extern "C" fn game_specialhimove(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.95);
    frame(lua_state, 12.6);
    if is_excute(agent) {
        GroundModule::set_passable_check(boma, true);
    }
}

unsafe extern "C" fn game_specialhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
        } else {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
        } else {
            WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            CancelModule::enable_cancel(boma);
        }
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_kagebunshin"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            EFFECT(agent, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            FLASH(agent, 1, 1, 1, 0.75);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..4 {
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
                FLASH(agent, 0.7, 0.7, 0.7, 0.5);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
                FLASH(agent, 0.67, 0, 0.78, 0.31);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
                COL_NORMAL(agent);
            }
        }
        wait(lua_state, 2.0);
    }
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS) {
            COL_NORMAL(agent);
        }
    }
}

unsafe extern "C" fn game_specialairappear(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 24.0, 361, 120, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 24.0, 361, 120, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("toel"), 24.0, 361, 120, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
    }
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS)
        && VarModule::get_float(boma.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE) >= 0.0 {
            DamageModule::heal(boma, -DamageModule::damage(boma, 0), 0);
            DamageModule::add_damage(boma, VarModule::get_float(boma.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE), 0);
            VarModule::set_float(boma.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE, -1.0);
        }
    }
}

unsafe extern "C" fn game_specialairlwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::lucario::instance::AURA_MAXIMUS)
        && VarModule::get_float(boma.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE) >= 0.0 {
            DamageModule::heal(boma, -DamageModule::damage(boma, 0), 0);
            DamageModule::add_damage(boma, VarModule::get_float(boma.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE), 0);
            VarModule::set_float(boma.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE, -1.0);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specialairs, Priority::Low);
    agent.acmd("game_specialsthrow", game_specialsthrow, Priority::Low);
    agent.acmd("effect_specialsthrow", effect_specialsthrow, Priority::Low);
    agent.acmd("game_specialhimove", game_specialhimove, Priority::Low);
    agent.acmd("game_specialhiend", game_specialhiend, Priority::Low);
    agent.acmd("game_specialairhiend", game_specialhiend, Priority::Low);
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("game_specialairappear", game_specialairappear, Priority::Low);
    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);
    agent.acmd("game_specialairlwend", game_specialairlwend, Priority::Low);
}