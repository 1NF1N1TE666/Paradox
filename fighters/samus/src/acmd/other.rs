use super::*;

unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
            FOOT_EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        } else {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_run(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    loop {
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 3, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.6);
                EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(agent, 0.2, 0.4, 5.0);
            } else {
                FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 22.0);
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.6);
                EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(agent, 0.2, 0.4, 5.0);
            } else {
                FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 20.0);
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.6);
                EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(agent, 0.2, 0.4, 5.0);
            } else {
                FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 22.0);
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 0.6);
                EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(agent, 0.2, 0.4, 5.0);
            } else {
                FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(lua_state, 20.0);
    }
}

unsafe extern "C" fn game_appeals(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let ice = VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        VarModule::set_flag(boma.object(), vars::samus::instance::ICE_MODE, !ice);
        suit_effect(boma, boma.object());
        boma.set_int(0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        EffectModule::remove_common(boma, Hash40::new("charge_max"));
        let eff_max = boma.get_int(*FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
        effect!(agent, MA_MSC_EFFECT_REMOVE, eff_max, 1);
    }
}

unsafe extern "C" fn effect_appeals(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samus_appeal_s"), Hash40::new("armr"), 7, 0, 0, 0, 0, 90, 1, true);
        LAST_EFFECT_SET_RATE(agent,2.25);
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            LAST_EFFECT_SET_COLOR(agent,0.0, 0.875, 1.25);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 7, 0, 0, 0, 0, 90, 1, true);
            LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);

            EFFECT_FOLLOW(agent,Hash40::new("sys_hit_ice"), Hash40::new("armr"), 8, 0, 0, 0, 0, 90, 0.2, true);
        }
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_sscope_bullet"),false,false);
    }
}

unsafe extern "C" fn sound_appeals(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_appeal_s01"));
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_samus_appeal_s03"));
        } else{
            PLAY_SE(agent, Hash40::new("se_common_frieze_ll"));
        }
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_samus_appeal_s03"));
        PLAY_SE(agent, Hash40::new("se_samus_appeal_s04"));
        STOP_SE(agent, Hash40::new("se_common_frieze_ll"));
    }
}

unsafe extern "C" fn expression_appeals(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        ArticleModule::change_motion(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("appeal_sl"), false, -1.0);
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            LinkModule::send_event_nodes(boma, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        let rumble = if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {Hash40::new("rbkind_15_iceberg_sp")} else {Hash40::new("rbkind_elecattacks")};
        ControlModule::set_rumble(boma, rumble, 40, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 168.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_shinesparkready(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FLASH(agent, 0.2, 0.4, 10.0, 0.5);
        BURN_COLOR(agent, 1, 1, 1, 0.9);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0.5);
        BURN_COLOR(agent, 1, 1, 1, 0.9);
        EFFECT_FOLLOW(agent, Hash40::new("sys_genesis_end"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.6);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deku_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 0.4, 10.0);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        FLASH(agent, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(agent, 1.0, 0.0, 4.0, 0.1);
    }
}

unsafe extern "C" fn sound_shinesparkready(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_samus_landing02"));
        PLAY_SE(agent, Hash40::new("se_item_magicball_warpout"));
        PLAY_SE(agent, Hash40::new("se_samus_catch"));
    }
}

unsafe extern "C" fn expression_shinesparkready(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_shinesparkstart(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 25.0);
    }
}

unsafe extern "C" fn effect_shinesparkstart(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 5.0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 4.0);
        FLASH(agent, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(agent, 1.0, 0.0, 4.0, 0.5);
        if !VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, -5, -90, 0, 0, 1.4, true);
            LAST_PARTICLE_SET_COLOR(agent, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, 3, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -90, 0, 0, 0.5, true);
            LAST_PARTICLE_SET_COLOR(agent, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
        }
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 5, -90, 0, 0, 1.4, true);
            LAST_PARTICLE_SET_COLOR(agent, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.4);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, -3, 0, 0, 0, 1.5, true);         
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, 3, -90, 0, 0, 0.5, true);
            LAST_PARTICLE_SET_COLOR(agent, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.4);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
        }
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, -5, -90, 0, 0, 1.4, true);
            LAST_PARTICLE_SET_COLOR(agent, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, 3, 0, 0, 0, 1.5, true);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -90, 0, 0, 0.5, true);
            LAST_PARTICLE_SET_COLOR(agent, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn sound_shinesparkstart(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_superscope_charge"));
    }
}

unsafe extern "C" fn expression_shinesparkstart(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_32_hold_lv1"), 0, true, 0);
        if VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_sphere") as i64);
            ItemModule::set_have_item_visibility(boma, false, 0);
            ItemModule::set_attach_item_visibility(boma, false, 0);
        }
    }
}

unsafe extern "C" fn game_shinesparkairloophi(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 200, 0, 0, 8.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_shinesparkairloophi(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 5.0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 4.0);
        FLASH(agent, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(agent, 1.0, 0.0, 4.0, 0.1);
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 3, 0, 180, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
    }
    loop {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(lua_state, 10.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(lua_state, 10.0);
    }
}

unsafe extern "C" fn game_shinesparkairloops(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        if !VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 200, 0, 0, 8.0, 0.0, 10.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 200, 0, 0, 6.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
    }
}

unsafe extern "C" fn effect_shinesparkairloops(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if !VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
        let stop_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let stop_speed_y = lua_bind::KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 5.0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 4.0);
            FLASH(agent, 1.0, 0.0, 4.0, 0.5);
            BURN_COLOR(agent, 1.0, 0.0, 4.0, 0.1);
            if stop_speed_y > 0.0 {
                EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 10, -5, -135, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
            } else if stop_speed_y < 0.0 {
                EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 10, -5, -45, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
            } else{
                EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 10, -5, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            if stop_speed_y > 0.0 {
                EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
            } else if stop_speed_y < 0.0 {
                EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
            } else{
                EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
            }
        }
        loop {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
                if stop_speed_y > 0.0 {
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
                } else if stop_speed_y < 0.0 {
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
                } else{
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
                    let ground_dist = GroundModule::get_distance_to_floor(boma, PostureModule::pos(boma), 90.0, true);
                    if ground_dist < 1.0 && ground_dist >= 0.0 {
                        LANDING_EFFECT(agent, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                    }
                }
            }
            wait(lua_state, 5.0);
            if is_excute(agent) {
                let ground_dist = GroundModule::get_distance_to_floor(boma, PostureModule::pos(boma), 90.0, true);
                if stop_speed_y == 0.0 && ground_dist < 1.0 && ground_dist >= 0.0 {
                    LANDING_EFFECT(agent, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                }
            }
            wait(lua_state, 5.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
                if stop_speed_y > 0.0 {
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
                } else if stop_speed_y < 0.0 {
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
                } else{
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
                    let ground_dist = GroundModule::get_distance_to_floor(boma, PostureModule::pos(boma), 90.0, true);
                    if ground_dist < 1.0 && ground_dist >= 0.0 {
                        LANDING_EFFECT(agent, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                    }
                }
            }
            wait(lua_state, 5.0);
            if is_excute(agent) {
                let ground_dist = GroundModule::get_distance_to_floor(boma, PostureModule::pos(boma), 90.0, true);
                if stop_speed_y == 0.0 && ground_dist < 1.0 && ground_dist >= 0.0 {
                    LANDING_EFFECT(agent, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                }
            }
            wait(lua_state, 5.0);
        }
    } else {
        let stop_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let stop_speed_x = lua_bind::KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
        let stop_speed_y = lua_bind::KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 3.0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 4.0);
            FLASH(agent, 1.0, 0.0, 4.0, 0.5);
            BURN_COLOR(agent, 1.0, 0.0, 4.0, 0.1);
            if stop_speed_y > 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 5, 2, -135, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -45, 0, 0, 1, true);
                } else {
                    EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 5, 0, 180, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 1, true);
                }
            } else if stop_speed_y < 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 1, 2, -45, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 45, 0, 0, 1, true);
                } else {
                    EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 90, 0, 0, 1, true);
                }
            } else{
                EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 3, 2, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            if stop_speed_y > 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, -45, 0, 0, 1, true);
                } else {
                    EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 1, true);
                }
            } else if stop_speed_y < 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, 45, 0, 0, 1, true);
                } else {
                    EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, 90, 0, 0, 1, true);
                }
            } else {
                EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
            }
        }
        loop {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                if stop_speed_y > 0.0 {
                    if stop_speed_x.abs() > 0.0 {
                        EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -45, 0, 0, 1, true);
                    } else {
                        EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 1, true);
                    }
                } else if stop_speed_y < 0.0 {
                    if stop_speed_x.abs() > 0.0 {
                        EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 45, 0, 0, 1, true);
                    } else {
                        EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 90, 0, 0, 1, true);
                    }
                } else{
                    EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                    let ground_dist = GroundModule::get_distance_to_floor(boma, PostureModule::pos(boma), 90.0, true);
                    if ground_dist < 1.0 && ground_dist >= 0.0 {
                        LANDING_EFFECT(agent, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                    }
                }
            }
            wait(lua_state, 5.0);
            if is_excute(agent) {
                let ground_dist = GroundModule::get_distance_to_floor(boma, PostureModule::pos(boma), 90.0, true);
                if stop_speed_y == 0.0 && ground_dist < 1.0 && ground_dist >= 0.0 {
                    LANDING_EFFECT(agent, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                }
            }
            wait(lua_state, 5.0);
        }
    }
}

unsafe extern "C" fn game_shinesparkairlooplw(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 270, 200, 0, 0, 8.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_shinesparkairlooplw(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 5.0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 4.0);
        FLASH(agent, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(agent, 1.0, 0.0, 4.0, 0.1);
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
    }
    loop {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(lua_state, 10.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(lua_state, 10.0);
    }
}

unsafe extern "C" fn sound_shinesparkairloop(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_superscope_chargeshot_l"));
    }
}

unsafe extern "C" fn expression_shinesparkairloop(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold2"), 0, true, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beaml"), 0, false, 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        if VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_sphere") as i64);
            ItemModule::set_have_item_visibility(boma, false, 0);
            ItemModule::set_attach_item_visibility(boma, false, 0);
        }
    } 
}

unsafe extern "C" fn game_shinesparkend(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_shinesparkend(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_drill"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(agent);
        BURN_COLOR_NORMAL(agent);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_shinesparkend(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_step_left_m"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_step_right_m"));
    }
}

unsafe extern "C" fn expression_shinesparkend(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 6);
    }
}

unsafe extern "C" fn game_shinesparkairend(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_GRAVITY);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_CONTROL);
    }
}

unsafe extern "C" fn effect_shinesparkairend(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(agent);
        BURN_COLOR_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_shinesparkairend(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_samus_jump02"));
    }
}

unsafe extern "C" fn game_shinesparkairceil(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_GRAVITY);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_CONTROL);
    }
}

unsafe extern "C" fn effect_shinesparkairceil(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(agent);
        BURN_COLOR_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_shinesparkairceil(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}

unsafe extern "C" fn expression_shinesparkairceil(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

unsafe extern "C" fn game_shinesparkwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_shinesparkwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(agent);
        BURN_COLOR_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_shinesparkwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}

unsafe extern "C" fn expression_shinesparkwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

unsafe extern "C" fn game_shinesparkairwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_GRAVITY);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_CONTROL);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_GRAVITY);
            SET_SPEED_EX(agent, -0.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            VarModule::on_flag(boma.object(), vars::samus::status::SHINESPARK_ENABLE_CONTROL);
        }
    }
}

unsafe extern "C" fn effect_shinesparkairwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(agent);
        BURN_COLOR_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_shinesparkairwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}

unsafe extern "C" fn expression_shinesparkairwall(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

unsafe extern "C" fn game_shinesparklanding(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_shinesparklanding(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(agent);
        BURN_COLOR_NORMAL(agent);
        LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_shinesparklanding(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}

unsafe extern "C" fn expression_shinesparklanding(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_dash", effect_dash, Priority::Low);
    agent.acmd("effect_run", effect_run, Priority::Low);
    agent.acmd("game_appealsl", game_appeals, Priority::Low);
    agent.acmd("effect_appealsl", effect_appeals, Priority::Low);
    agent.acmd("sound_appealsl", sound_appeals, Priority::Low);
    agent.acmd("expression_appealsl", expression_appeals, Priority::Low);
    agent.acmd("game_appealsr", game_appeals, Priority::Low);
    agent.acmd("effect_appealsr", effect_appeals, Priority::Low);
    agent.acmd("sound_appealsr", sound_appeals, Priority::Low);
    agent.acmd("expression_appealsr", expression_appeals, Priority::Low);
    agent.acmd("game_shinesparkready", acmd_stub, Priority::Low);
    agent.acmd("effect_shinesparkready", effect_shinesparkready, Priority::Low);
    agent.acmd("sound_shinesparkready", sound_shinesparkready, Priority::Low);
    agent.acmd("expression_shinesparkready", expression_shinesparkready, Priority::Low);
    agent.acmd("game_shinesparkstart", game_shinesparkstart, Priority::Low);
    agent.acmd("effect_shinesparkstart", effect_shinesparkstart, Priority::Low);
    agent.acmd("sound_shinesparkstart", sound_shinesparkstart, Priority::Low);
    agent.acmd("expression_shinesparkstart", expression_shinesparkstart, Priority::Low);
    agent.acmd("game_shinesparkairstart", game_shinesparkstart, Priority::Low);
    agent.acmd("effect_shinesparkairstart", effect_shinesparkstart, Priority::Low);
    agent.acmd("sound_shinesparkairstart", sound_shinesparkstart, Priority::Low);
    agent.acmd("expression_shinesparkairstart", expression_shinesparkstart, Priority::Low);
    agent.acmd("game_shinesparkairloophi", game_shinesparkairloophi, Priority::Low);
    agent.acmd("effect_shinesparkairloophi", effect_shinesparkairloophi, Priority::Low);
    agent.acmd("sound_shinesparkairloophi", sound_shinesparkairloop, Priority::Low);
    agent.acmd("expression_shinesparkairloophi", expression_shinesparkairloop, Priority::Low);
    agent.acmd("game_shinesparkairloops", game_shinesparkairloops, Priority::Low);
    agent.acmd("effect_shinesparkairloops", effect_shinesparkairloops, Priority::Low);
    agent.acmd("sound_shinesparkairloops", sound_shinesparkairloop, Priority::Low);
    agent.acmd("expression_shinesparkairloops", expression_shinesparkairloop, Priority::Low);
    agent.acmd("game_shinesparkairlooplw", game_shinesparkairlooplw, Priority::Low);
    agent.acmd("effect_shinesparkairlooplw", effect_shinesparkairlooplw, Priority::Low);
    agent.acmd("sound_shinesparkairlooplw", sound_shinesparkairloop, Priority::Low);
    agent.acmd("expression_shinesparkairlooplw", expression_shinesparkairloop, Priority::Low);
    agent.acmd("game_shinesparkend", game_shinesparkend, Priority::Low);
    agent.acmd("effect_shinesparkend", effect_shinesparkend, Priority::Low);
    agent.acmd("sound_shinesparkend", sound_shinesparkend, Priority::Low);
    agent.acmd("expression_shinesparkend", expression_shinesparkend, Priority::Low);
    agent.acmd("game_shinesparkairend", game_shinesparkairend, Priority::Low);
    agent.acmd("effect_shinesparkairend", effect_shinesparkairend, Priority::Low);
    agent.acmd("sound_shinesparkairend", sound_shinesparkairend, Priority::Low);
    agent.acmd("expression_shinesparkairend", acmd_stub, Priority::Low);
    agent.acmd("game_shinesparkairceil", game_shinesparkairceil, Priority::Low);
    agent.acmd("effect_shinesparkairceil", effect_shinesparkairceil, Priority::Low);
    agent.acmd("sound_shinesparkairceil", sound_shinesparkairceil, Priority::Low);
    agent.acmd("expression_shinesparkairceil", expression_shinesparkairceil, Priority::Low);
    agent.acmd("game_shinesparkwall", game_shinesparkwall, Priority::Low);
    agent.acmd("effect_shinesparkwall", effect_shinesparkwall, Priority::Low);
    agent.acmd("sound_shinesparkwall", sound_shinesparkwall, Priority::Low);
    agent.acmd("expression_shinesparkwall", expression_shinesparkwall, Priority::Low);
    agent.acmd("game_shinesparkairwall", game_shinesparkairwall, Priority::Low);
    agent.acmd("effect_shinesparkairwall", effect_shinesparkairwall, Priority::Low);
    agent.acmd("sound_shinesparkairwall", sound_shinesparkairwall, Priority::Low);
    agent.acmd("expression_shinesparkairwall", expression_shinesparkairwall, Priority::Low);
    agent.acmd("game_shinesparklanding", game_shinesparklanding, Priority::Low);
    agent.acmd("effect_shinesparklanding", effect_shinesparklanding, Priority::Low);
    agent.acmd("sound_shinesparklanding", sound_shinesparklanding, Priority::Low);
    agent.acmd("expression_shinesparklanding", expression_shinesparklanding, Priority::Low);
}