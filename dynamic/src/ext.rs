use crate::consts::{globals::*, vars};
use bitflags::bitflags;
use modular_bitfield::specifiers::*;
use smash::app::{
    self, lua_bind::*, FighterKineticEnergyController, FighterKineticEnergyGravity, FighterKineticEnergyMotion, *,
};
use smash::lib::{lua_const::*, *};
use smash::lua2cpp::*;
use smash::phx::*;
use crate::VarModule;
use crate::consts::*;

pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
    fn mag(&self) -> f32;
    fn normalize(&self) -> Self;
}

pub trait Vec4Ext {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

pub trait Hash40Ext {
    fn to_hash(self) -> Hash40;
}

impl Hash40Ext for Hash40 {
    fn to_hash(self) -> Hash40 {
        self
    }
}

impl Hash40Ext for u64 {
    fn to_hash(self) -> Hash40 {
        Hash40::new_raw(self)
    }
}

impl Hash40Ext for &str {
    fn to_hash(self) -> Hash40 {
        Hash40::new(self)
    }
}

impl Vec2Ext for Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

impl Vec4Ext for Vector4f {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Copy, Clone)]
pub enum CommandCat {
    Cat1(Cat1),
    Cat2(Cat2),
    Cat3(Cat3),
    Cat4(Cat4),
    CatHdr(CatHdr),
}

impl Into<CommandCat> for Cat1 {
    fn into(self) -> CommandCat {
        CommandCat::Cat1(self)
    }
}

impl Into<CommandCat> for Cat2 {
    fn into(self) -> CommandCat {
        CommandCat::Cat2(self)
    }
}

impl Into<CommandCat> for Cat3 {
    fn into(self) -> CommandCat {
        CommandCat::Cat3(self)
    }
}

impl Into<CommandCat> for Cat4 {
    fn into(self) -> CommandCat {
        CommandCat::Cat4(self)
    }
}

impl Into<CommandCat> for CatHdr {
    fn into(self) -> CommandCat {
        CommandCat::CatHdr(self)
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    pub struct Cat1: i32 {
        const AttackN       = 0x1;
        const AttackS3      = 0x2;
        const AttackHi3     = 0x4;
        const AttackLw3     = 0x8;
        const AttackS4      = 0x10;
        const AttackHi4     = 0x20;
        const AttackLw4     = 0x40;
        const AttackAirN    = 0x80;
        const AttackAirF    = 0x100;
        const AttackAirB    = 0x200;
        const AttackAirHi   = 0x400;
        const AttackAirLw   = 0x800;
        const SpecialN      = 0x1000;
        const SpecialS      = 0x2000;
        const SpecialHi     = 0x4000;
        const SpecialLw     = 0x8000;
        const SpecialAny    = 0xF000;
        const Walk          = 0x10000;
        const Dash          = 0x20000;
        const Turn          = 0x40000;
        const TurnDash      = 0x80000;
        const Jump          = 0x100000;
        const JumpButton    = 0x200000;
        const AirEscape     = 0x400000;
        const Squat         = 0x800000;
        const Escape        = 0x1000000;
        const EscapeF       = 0x2000000;
        const EscapeB       = 0x4000000;
        const WallJumpLeft  = 0x8000000;
        const WallJumpRight = 0x10000000;
        const Catch         = 0x20000000;
        const NoCmd         = 0x40000000;
    }

    #[derive(Copy, Clone)]
    pub struct Cat2: i32 {
        const AppealSL            = 0x1;
        const AppealSR            = 0x2;
        const AppealHi            = 0x4;
        const AppealLw            = 0x8;
        const AppealSmash         = 0x10;
        const AppealAll           = 0x1F;
        const AttackDashAttackHi4 = 0x20;
        const FallJump            = 0x40;
        const DashAttackS4        = 0x80;
        const DamageFallToFall    = 0x100;
        const DownToDownStandFB   = 0x200;
        const DownToDownStand     = 0x400;
        const GuardToPass         = 0x800;
        const SquatToSquatF       = 0x1000;
        const SquatToSquatB       = 0x2000;
        const TurnToEscapeF       = 0x4000;
        const TurnToEscapeB       = 0x8000;
        const StickEscapeF        = 0x10000;
        const StickEscapeB        = 0x20000;
        const StickEscape         = 0x40000;
        const SpecialNReverseLR   = 0x80000;
        const ThrowF              = 0x100000;
        const ThrowB              = 0x200000;
        const ThrowHi             = 0x400000;
        const ThrowLw             = 0x800000;
        const CommonGuard         = 0x1000000;
        const AirLasso            = 0x2000000;
        const AttackN2            = 0x4000000;
        const FinalReverseLR      = 0x8000000;
    }

    #[derive(Copy, Clone)]
    pub struct Cat3: i32 {
        const ItemLightThrowFB4    = 0x1;
        const ItemLightThrowHi4    = 0x2;
        const ItemLightThrowLw4    = 0x4;
        const ItemLightThrowHi     = 0x8;
        const ItemLightThrowLw     = 0x10;
        const ItemLightDrop        = 0x20;
        const ItemLightThrowFB     = 0x40;
        const ItemLightThrowAirFB  = 0x80;
        const ItemLightThrowAirFB4 = 0x100;
        const ItemLightThrowAirHi  = 0x200;
        const ItemLightThrowAirHi4 = 0x400;
        const ItemLightThrowAirLw  = 0x800;
        const ItemLightThrowAirLw4 = 0x1000;
        const ItemLightDropAir     = 0x2000;
        const ItemHeavyThrowFB     = 0x4000;
        const ItemGetAir           = 0x8000;
        const SpecialSSmash        = 0x10000;
        const SpecialSSmashDash    = 0x20000;

        const ItemLightThrow       = 0x58;
        const ItemLightThrowAir    = 0xA80;
        const ItemLightThrow4      = 0x7;
        const ItemLightThrow4Air   = 0x1500;
        const ItemLightThrowAll    = 0x5F;
        const ItemLightThrowAirAll = 0x1F80;
    }

    #[derive(Copy, Clone)]
    pub struct Cat4: i32 {
        const SpecialNCommand       = 0x1;
        const SpecialN2Command      = 0x2;
        const SpecialSCommand       = 0x4;
        const SpecialHiCommand      = 0x8;
        const Command6N6            = 0x10;
        const Command4N4            = 0x20;
        const AttackCommand1        = 0x40;
        const SpecialHi2Command     = 0x80;
        const SuperSpecialCommand   = 0x100;
        const SuperSpecialRCommand  = 0x200;
        const SuperSpecial2Command  = 0x400;
        const SuperSpecial2RCommand = 0x800;
        const Command623NB          = 0x1000;
        const Command623Strict      = 0x2000;
        const Command623ALong       = 0x4000;
        const Command623BLong       = 0x8000;
        const Command623A           = 0x10000;
        const Command2              = 0x20000;
        const Command3              = 0x40000;
        const Command1              = 0x80000;
        const Command6              = 0x100000;
        const Command4              = 0x200000;
        const Command8              = 0x400000;
        const Command9              = 0x800000;
        const Command7              = 0x1000000;
        const Command6N6AB          = 0x2000000;
        const Command323Catch       = 0x4000000;
    }

    #[derive(Copy, Clone)]
    pub struct CatHdr: i32 {
        const Wavedash = 0x1;
    }

    #[derive(Copy, Clone)]
    pub struct PadFlag: i32 {
        const AttackTrigger  = 0x1;
        const AttrckRelease  = 0x2;
        const SpecialTrigger = 0x4;
        const SpecialRelease = 0x8;
        const JumpTrigger    = 0x10;
        const JumpRelease    = 0x20;
        const GuardTrigger   = 0x40;
        const GuardRelease   = 0x80;
    }

    #[derive(Copy, Clone)]
    pub struct Buttons: i32 {
        const Attack      = 0x1;
        const Special     = 0x2;
        const Jump        = 0x4;
        const Guard       = 0x8;
        const Catch       = 0x10;
        const Smash       = 0x20;
        const JumpMini    = 0x40;
        const CStickOn    = 0x80;
        const StockShare  = 0x100;
        const AttackRaw   = 0x200;
        const AppealHi    = 0x400;
        const SpecialRaw  = 0x800;
        const AppealLw    = 0x1000;
        const AppealSL    = 0x2000;
        const AppealSR    = 0x4000;
        const FlickJump   = 0x8000;
        const GuardHold   = 0x10000;
        const SpecialRaw2 = 0x20000;
        const TiltAttack  = 0x80000;
        const CStickOverride = 0x200000;

        const SpecialAll  = 0x20802;
        const AttackAll   = 0x201;
        const AppealAll   = 0x7400;
    }
}

impl Cat1 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { Cat1::from_bits_retain(ControlModule::get_command_flag_cat(boma, 0)) }
    }
}

impl Cat2 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { Cat2::from_bits_retain(ControlModule::get_command_flag_cat(boma, 1)) }
    }
}

impl Cat3 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { Cat3::from_bits_retain(ControlModule::get_command_flag_cat(boma, 2)) }
    }
}

impl Cat4 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { Cat4::from_bits_retain(ControlModule::get_command_flag_cat(boma, 3)) }
    }
}

impl CatHdr {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { CatHdr::from_bits_retain(ControlModule::get_command_flag_cat(boma, 4)) }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AerialKind {
    Nair,
    Fair,
    Bair,
    Uair,
    Dair,
}

pub trait MainShift {
    fn main_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue,
    ) -> L2CValue;
}

pub trait FastShift {
    fn fast_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterBase) -> L2CValue,
    ) -> L2CValue;
}

impl MainShift for L2CFighterCommon {
    fn main_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue,
    ) -> L2CValue {
        unsafe { self.sub_shift_status_main(L2CValue::Ptr(new_main as *const () as _)) }
    }
}

impl FastShift for L2CFighterBase {
    fn fast_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterBase) -> L2CValue,
    ) -> L2CValue {
        unsafe { self.fastshift(L2CValue::Ptr(new_main as *const () as _)) }
    }
}

pub trait BomaExt {
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool;
    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn stick_x(&mut self) -> f32;
    unsafe fn stick_y(&mut self) -> f32;
    unsafe fn prev_stick_x(&mut self) -> f32;
    unsafe fn prev_stick_y(&mut self) -> f32;
    unsafe fn is_input_jump(&mut self) -> bool;
    unsafe fn get_aerial(&mut self) -> Option<AerialKind>;
    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f);
    unsafe fn is_stick_forward(&mut self) -> bool;
    unsafe fn is_stick_backward(&mut self) -> bool;
    unsafe fn left_stick_x(&mut self) -> f32;
    unsafe fn prev_left_stick_x(&mut self) -> f32;
    unsafe fn left_stick_y(&mut self) -> f32;
    unsafe fn prev_left_stick_y(&mut self) -> f32;
    unsafe fn right_stick_x(&mut self) -> f32;
    unsafe fn prev_right_stick_x(&mut self) -> f32;
    unsafe fn right_stick_y(&mut self) -> f32;
    unsafe fn prev_right_stick_y(&mut self) -> f32;
    unsafe fn check_hold_input(&mut self, start_frame: i32, end_frame: i32, input: Buttons) -> bool;
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_motion(&mut self, motion: Hash40) -> bool;
    unsafe fn is_motion_one_of(&mut self, motions: &[Hash40]) -> bool;
    unsafe fn status(&mut self) -> i32;
    unsafe fn lr(&mut self) -> f32;
    unsafe fn get_num_used_jumps(&mut self) -> i32;
    unsafe fn get_jump_count_max(&mut self) -> i32;
    unsafe fn motion_frame(&mut self) -> f32;
    unsafe fn set_rate(&mut self, motion_rate: f32);
    unsafe fn is_in_hitlag(&mut self) -> bool;
    unsafe fn status_frame(&mut self) -> i32;
    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;
    unsafe fn set_status_kind_interrupt(&mut self, kind: i32);
    unsafe fn get_status_by_situation(&mut self, ground_status: i32, air_status: i32) -> i32;
    unsafe fn change_status_by_situation(&mut self, ground_status: i32, air_status: i32, repeat: bool) -> i32;
    unsafe fn get_motion_by_situation(&mut self, ground_motion: &str, air_motion: &str) -> Hash40;
    unsafe fn change_motion_by_situation(&mut self, ground_motion: &str, air_motion: &str, start_frame: f32, rate: f32, arg5: bool, arg6: f32, arg7: bool, arg8: bool) -> i32;
    unsafe fn change_motion_inherit_frame_by_situation(&mut self, ground_motion: &str, air_motion: &str, frame_offset: f32, rate: f32, arg5: f32, arg6: bool, arg7: bool) -> i32;
    unsafe fn change_motion_inherit_frame_keep_rate_by_situation(&mut self, ground_motion: &str, air_motion: &str, frame_offset: f32, rate: f32, arg5: f32) -> i32;
    unsafe fn get_hash_by_situation(&mut self, ground_hash: &str, air_hash: &str) -> Hash40;
    unsafe fn change_kinetic_by_situation(&mut self, ground_kinetic_type: i32, air_kinetic_type: i32) -> i32;
    unsafe fn ground_correct_by_situation(&mut self, ground_correct_kind: i32, air_correct_kind: i32) -> i32;
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn is_item(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
    unsafe fn get_grabbed_opponent_boma(&mut self) -> &mut BattleObjectModuleAccessor;
    unsafe fn get_grabber_boma(&mut self) -> &mut BattleObjectModuleAccessor;
    unsafe fn get_owner_boma(&mut self) -> &mut BattleObjectModuleAccessor;
    unsafe fn get_int(&mut self, what: i32) -> i32;
    unsafe fn inc_int(&mut self, what: i32);
    unsafe fn dec_int(&mut self, what: i32);
    unsafe fn get_float(&mut self, what: i32) -> f32;
    unsafe fn get_int64(&mut self, what: i32) -> u64;
    unsafe fn is_flag(&mut self, what: i32) -> bool;
    unsafe fn set_int(&mut self, value: i32, what: i32);
    unsafe fn set_float(&mut self, value: f32, what: i32);
    unsafe fn set_int64(&mut self, value: i64, what: i32);
    unsafe fn set_flag(&mut self, value: bool, what: i32);
    unsafe fn on_flag(&mut self, what: i32);
    unsafe fn off_flag(&mut self, what: i32);
    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32;
    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32;
    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64;
    unsafe fn set_int_from_param(
        &mut self,
        what: i32,
        object: impl Hash40Ext,
        param: impl Hash40Ext,
    );
    unsafe fn set_float_from_param(
        &mut self,
        what: i32,
        object: impl Hash40Ext,
        param: impl Hash40Ext,
    );
    unsafe fn set_int64_from_param(
        &mut self,
        what: i32,
        object: impl Hash40Ext,
        param: impl Hash40Ext,
    );
    unsafe fn enable_transition_term(&mut self, arg2: i32);
    unsafe fn enable_transition_term_many(&mut self, arg2: &[i32]);
    unsafe fn unable_transition_term(&mut self, arg2: i32);
    unsafe fn unable_transition_term_many(&mut self, arg2: &[i32]);
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion;
    unsafe fn get_gravity_energy(&mut self) -> &mut FighterKineticEnergyGravity;
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController;
    unsafe fn handle_waveland(&mut self, require_airdodge: bool) -> bool;
    unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_center_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn get_front_cliff_hangdata(&mut self) -> Vector2f;
    unsafe fn get_back_cliff_hangdata(&mut self) -> Vector2f;
    unsafe fn get_center_cliff_hangdata(&mut self) -> Vector2f;
    unsafe fn check_jump_cancel(&mut self, update_lr: bool) -> bool;
    unsafe fn check_airdodge_cancel(&mut self) -> bool;
    unsafe fn check_aerial_cancel(&mut self) -> bool;
    unsafe fn check_dash_cancel(&mut self) -> bool;
    unsafe fn check_wall_jump_cancel(&mut self) -> bool;
    unsafe fn check_land_cancel(&mut self, landing_lag: Option<f32>) -> bool;
    unsafe fn paradox_funcs(&mut self);
    unsafe fn try_pickup_item(&mut self, range: f32, bone: Option<Hash40>, offset: Option<&Vector2f>) -> Option<&mut BattleObjectModuleAccessor> ;
    unsafe fn get_player_idx_from_boma(&mut self) -> i32;
    unsafe fn set_command_input_button(&mut self, command: usize, buttons: u8);
    unsafe fn clone_command_input(&mut self, command: usize, replace_command: usize);
}

impl BomaExt for BattleObjectModuleAccessor {

    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat),
        }
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).contains(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).contains(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).contains(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).contains(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat),
        }
    }

    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool {
        PadFlag::from_bits_retain(ControlModule::get_pad_flag(self)).intersects(pad_flag)
    }

    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_retain(ControlModule::get_button(self)).intersects(buttons)
    }

    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool {
        !self.is_button_on(buttons)
    }

    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_retain(ControlModule::get_trigger(self)).intersects(buttons)
    }

    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_retain(ControlModule::get_release(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_retain(ControlModule::get_button_prev(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool {
        !self.was_prev_button_on(buttons)
    }

    unsafe fn stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_x(self);
    }

    unsafe fn stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_y(self);
    }

    unsafe fn prev_stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_prev_x(self);
    }

    unsafe fn prev_stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_prev_y(self);
    }

    unsafe fn is_input_jump(&mut self) -> bool {
        if self.is_cat_flag(Cat1::Jump) && ControlModule::is_enable_flick_jump(self) {
            WorkModule::set_int(
                self,
                1,
                *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE,
            );
            return true;
        }

        return self.is_cat_flag(Cat1::JumpButton);
    }

    unsafe fn is_stick_forward(&mut self) -> bool {
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x * PostureModule::lr(self) > 0. {
                return true;
            }
        }
        return false;
    }

    unsafe fn is_stick_backward(&mut self) -> bool {
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x * PostureModule::lr(self) < 0. {
                return true;
            }
        }
        return false;
    }

    unsafe fn left_stick_x(&mut self) -> f32 {
        if self.is_button_on(Buttons::CStickOverride) {
            return ControlModule::get_sub_stick_x(self);
        } else {
            return ControlModule::get_stick_x(self);
        }
    }

    unsafe fn prev_left_stick_x(&mut self) -> f32 {
        if self.was_prev_button_on(Buttons::CStickOverride) {
            return ControlModule::get_sub_stick_prev_x(self);
        } else {
            return ControlModule::get_stick_prev_x(self);
        }
    }

    unsafe fn left_stick_y(&mut self) -> f32 {
        if self.is_button_on(Buttons::CStickOverride) {
            return ControlModule::get_sub_stick_y(self);
        } else {
            return ControlModule::get_stick_y(self);
        }
    }

    unsafe fn prev_left_stick_y(&mut self) -> f32 {
        if self.was_prev_button_on(Buttons::CStickOverride) {
            return ControlModule::get_sub_stick_prev_y(self);
        } else {
            return ControlModule::get_stick_prev_y(self);
        }
    }

    unsafe fn right_stick_x(&mut self) -> f32 {
        if self.is_button_on(Buttons::CStickOverride) {
            return ControlModule::get_stick_x(self);
        } else {
            return ControlModule::get_sub_stick_x(self);
        }
    }

    unsafe fn prev_right_stick_x(&mut self) -> f32 {
        if self.was_prev_button_on(Buttons::CStickOverride) {
            return ControlModule::get_stick_prev_x(self);
        } else {
            return ControlModule::get_sub_stick_prev_x(self);
        }
    }

    unsafe fn right_stick_y(&mut self) -> f32 {
        if self.is_button_on(Buttons::CStickOverride) {
            return ControlModule::get_stick_y(self);
        } else {
            return ControlModule::get_sub_stick_y(self);
        }
    }

    unsafe fn prev_right_stick_y(&mut self) -> f32 {
        if self.was_prev_button_on(Buttons::CStickOverride) {
            return ControlModule::get_stick_prev_y(self);
        } else {
            return ControlModule::get_sub_stick_prev_y(self);
        }
    }

    unsafe fn check_hold_input(&mut self, start_frame: i32, end_frame: i32, input: Buttons) -> bool {
        if !(start_frame..=end_frame).contains(&self.status_frame()) {
            return false;
        }

        if self.status_frame() == start_frame && !self.is_button_off(input) {
            VarModule::on_flag(self.object(), vars::common::status::CHECK_HOLD_INPUT);
        }

        if VarModule::is_flag(self.object(), vars::common::status::CHECK_HOLD_INPUT) {
            // if we are still checking for the hold and we are ready to end the check
            if self.status_frame() == end_frame {
                VarModule::off_flag(self.object(), vars::common::status::CHECK_HOLD_INPUT);
                return true;
            }

            if self.is_button_release(input) {
                VarModule::off_flag(self.object(), vars::common::status::CHECK_HOLD_INPUT);
                return false;
            }
        }

        return false;
    }

    unsafe fn get_aerial(&mut self) -> Option<AerialKind> {
        if self.is_cat_flag(Cat1::AttackHi3 | Cat1::AttackHi4) {
            Some(AerialKind::Uair)
        } else if self.is_cat_flag(Cat1::AttackLw3 | Cat1::AttackLw4) {
            Some(AerialKind::Dair)
        } else if self.is_cat_flag(Cat1::AttackS3 | Cat1::AttackS4) {
            if self.is_stick_backward() {
                Some(AerialKind::Bair)
            } else {
                Some(AerialKind::Fair)
            }
        } else if self.is_cat_flag(Cat1::AttackN | Cat1::AttackAirN) {
            Some(AerialKind::Nair)
        } else {
            None
        }
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return StatusModule::status_kind(self) == kind;
    }

    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }

    unsafe fn is_prev_status(&mut self, kind: i32) -> bool {
        return StatusModule::prev_status_kind(self, 0) == kind;
    }

    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::prev_status_kind(self, 0);
        return kinds.contains(&kind);
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return StatusModule::situation_kind(self) == kind;
    }

    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool {
        return StatusModule::prev_situation_kind(self) == kind;
    }

    unsafe fn is_motion(&mut self, kind: Hash40) -> bool {
        return MotionModule::motion_kind(self) == kind.hash;
    }

    unsafe fn set_rate(&mut self, motion_rate: f32) {
        MotionModule::set_rate(self, motion_rate);
    }

    unsafe fn is_motion_one_of(&mut self, kinds: &[Hash40]) -> bool {
        let kind = MotionModule::motion_kind(self);
        return kinds.contains(&Hash40::new_raw(kind));
    }

    unsafe fn motion_frame(&mut self) -> f32 {
        return MotionModule::frame(self);
    }

    unsafe fn is_in_hitlag(&mut self) -> bool {
        let hitlag_frame = WorkModule::get_int(
            self,
            *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME,
        );
        if hitlag_frame > 0 {
            return true;
        }
        return false;
    }

    unsafe fn status_frame(&mut self) -> i32 {
        return crate::util::get_fighter_common_from_accessor(self).global_table[CURRENT_FRAME]
            .get_i32();
    }

    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32 {
        return StatusModule::change_status_request_from_script(self, kind, repeat) as i32;
    }

    unsafe fn set_status_kind_interrupt(&mut self, kind: i32) {
        StatusModule::set_status_kind_interrupt(self, kind);
        let status_module = *(self as *const BattleObjectModuleAccessor as *const u64).add(0x8);
        *((status_module + 0x98) as *mut i32) = kind;  // StatusModule::status_kind
        *((status_module + 0x9c) as *mut i32) = kind;  // StatusModule::status_kind_next
        crate::util::get_fighter_common_from_accessor(self).global_table[STATUS_KIND].assign(&L2CValue::I32(kind));
    }

    unsafe fn get_status_by_situation(&mut self, ground_status: i32, air_status: i32) -> i32 {
        return if self.is_situation(*SITUATION_KIND_GROUND) { ground_status } else { air_status };
    }

    unsafe fn change_status_by_situation(&mut self, ground_status: i32, air_status: i32, repeat: bool) -> i32 {
        return if self.is_situation(*SITUATION_KIND_GROUND) { self.change_status_req(ground_status, repeat) } else { self.change_status_req(air_status, repeat) };
    }

    unsafe fn get_motion_by_situation(&mut self, ground_motion: &str, air_motion: &str) -> Hash40 {
        return if self.is_situation(*SITUATION_KIND_GROUND) { Hash40::new(ground_motion) } else { Hash40::new(air_motion) };
    }

    unsafe fn change_motion_by_situation(&mut self, ground_motion: &str, air_motion: &str, start_frame: f32, rate: f32, arg5: bool, arg6: f32, arg7: bool, arg8: bool) -> i32 {
        let motion = if self.is_situation(*SITUATION_KIND_GROUND) { Hash40::new(ground_motion) } else { Hash40::new(air_motion) };
        return MotionModule::change_motion(self, motion, start_frame, rate, arg5, arg6, arg7, arg8) as i32;
    }

    unsafe fn change_motion_inherit_frame_by_situation(&mut self, ground_motion: &str, air_motion: &str, start_frame: f32, rate: f32, arg5: f32, arg6: bool, arg7: bool) -> i32 {
        let motion = if self.is_situation(*SITUATION_KIND_GROUND) { Hash40::new(ground_motion) } else { Hash40::new(air_motion) };
        return MotionModule::change_motion_inherit_frame(self, motion, start_frame, rate, arg5, arg6, arg7) as i32;
    }

    unsafe fn change_motion_inherit_frame_keep_rate_by_situation(&mut self, ground_motion: &str, air_motion: &str, frame_offset: f32, rate: f32, arg5: f32) -> i32 {
        let motion = if self.is_situation(*SITUATION_KIND_GROUND) { Hash40::new(ground_motion) } else { Hash40::new(air_motion) };
        return MotionModule::change_motion_inherit_frame_keep_rate(self, motion, frame_offset, rate, arg5) as i32;
    }

    unsafe fn get_hash_by_situation(&mut self, ground_hash: &str, air_hash: &str) -> Hash40 {
        return if self.is_situation(*SITUATION_KIND_GROUND) { Hash40::new(ground_hash) } else { Hash40::new(air_hash) };
    }

    unsafe fn change_kinetic_by_situation(&mut self, ground_kinetic_kind: i32, air_kinetic_kind: i32) -> i32 {
        let kinetic = if self.is_situation(*SITUATION_KIND_GROUND) { ground_kinetic_kind } else { air_kinetic_kind };
        return KineticModule::change_kinetic(self, kinetic);
    }

    unsafe fn ground_correct_by_situation(&mut self, ground_correct_type: i32, air_correct_type: i32) -> i32 {
        let ground_correct = if self.is_situation(*SITUATION_KIND_GROUND) { GroundCorrectKind(ground_correct_type) } else { GroundCorrectKind(air_correct_type) };
        return GroundModule::correct(self, ground_correct) as i32;
    }

    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }

    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }

    unsafe fn is_item(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_ITEM;
    }

    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }

    unsafe fn get_grabbed_opponent_boma(&mut self) -> &mut BattleObjectModuleAccessor {
        let opponent_id = LinkModule::get_node_object_id(self, *LINK_NO_CAPTURE) as u32;
        let opponent_object = super::util::get_battle_object_from_id(opponent_id);
        &mut *(*opponent_object).module_accessor
    }

    unsafe fn get_grabber_boma(&mut self) -> &mut BattleObjectModuleAccessor {
        let opponent_id = LinkModule::get_parent_object_id(self, *LINK_NO_CAPTURE) as u32;
        let opponent_object = super::util::get_battle_object_from_id(opponent_id);
        &mut *(*opponent_object).module_accessor
    }

    unsafe fn get_owner_boma(&mut self) -> &mut BattleObjectModuleAccessor {
        return &mut *sv_battle_object::module_accessor((WorkModule::get_int(self, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32);
    }

    unsafe fn get_num_used_jumps(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    unsafe fn get_jump_count_max(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    }

    unsafe fn get_int(&mut self, what: i32) -> i32 {
        WorkModule::get_int(self, what)
    }

    unsafe fn inc_int(&mut self, what: i32) {
        WorkModule::inc_int(self, what)
    }

    unsafe fn dec_int(&mut self, what: i32) {
        WorkModule::dec_int(self, what)
    }

    unsafe fn get_float(&mut self, what: i32) -> f32 {
        WorkModule::get_float(self, what)
    }

    unsafe fn get_int64(&mut self, what: i32) -> u64 {
        WorkModule::get_int64(self, what)
    }

    unsafe fn is_flag(&mut self, what: i32) -> bool {
        WorkModule::is_flag(self, what)
    }

    unsafe fn set_int(&mut self, value: i32, what: i32) {
        WorkModule::set_int(self, value, what)
    }

    unsafe fn set_int_from_param(
        &mut self,
        what: i32,
        object: impl Hash40Ext,
        param: impl Hash40Ext,
    ) {
        let int = WorkModule::get_param_int(self, object.to_hash().hash, param.to_hash().hash);
        WorkModule::set_int(self, int, what);
    }

    unsafe fn set_float(&mut self, value: f32, what: i32) {
        WorkModule::set_float(self, value, what)
    }

    unsafe fn set_float_from_param(
        &mut self,
        what: i32,
        object: impl Hash40Ext,
        param: impl Hash40Ext,
    ) {
        let float = WorkModule::get_param_float(self, object.to_hash().hash, param.to_hash().hash);
        WorkModule::set_float(self, float, what);
    }

    unsafe fn set_int64(&mut self, value: i64, what: i32) {
        WorkModule::set_int64(self, value, what)
    }

    unsafe fn set_int64_from_param(
        &mut self,
        what: i32,
        object: impl Hash40Ext,
        param: impl Hash40Ext,
    ) {
        let int = WorkModule::get_param_int64(self, object.to_hash().hash, param.to_hash().hash);
        WorkModule::set_int64(self, int as i64, what);
    }

    unsafe fn set_flag(&mut self, value: bool, what: i32) {
        WorkModule::set_flag(self, value, what)
    }

    unsafe fn on_flag(&mut self, what: i32) {
        WorkModule::on_flag(self, what)
    }

    unsafe fn off_flag(&mut self, what: i32) {
        WorkModule::off_flag(self, what)
    }

    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32 {
        WorkModule::get_param_int(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32 {
        let obj = obj.into();
        let field = field.into();
        WorkModule::get_param_float(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64 {
        let obj = obj.into();
        let field = field.into();
        WorkModule::get_param_int64(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn enable_transition_term(&mut self, arg2: i32) {
        WorkModule::enable_transition_term(self, arg2)
    }
    unsafe fn enable_transition_term_many(&mut self, arg2: &[i32]) {
        for term in arg2.iter() {
            WorkModule::enable_transition_term(self, *term);
        }
    }
    unsafe fn unable_transition_term(&mut self, arg2: i32) {
        WorkModule::unable_transition_term(self, arg2)
    }
    unsafe fn unable_transition_term_many(&mut self, arg2: &[i32]) {
        for term in arg2.iter() {
            WorkModule::unable_transition_term(self, *term);
        }
    }

    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f) {
        ModelModule::set_joint_rotate(
            self,
            Hash40::new(&bone_name),
            &rotation,
            MotionNodeRotateCompose {
                _address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8,
            },
            MotionNodeRotateOrder {
                _address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8,
            },
        )
    }

    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion {
        std::mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(
            self,
            *FIGHTER_KINETIC_ENERGY_ID_MOTION,
        ))
    }

    unsafe fn get_gravity_energy(&mut self) -> &mut FighterKineticEnergyGravity {
        std::mem::transmute::<u64, &mut app::FighterKineticEnergyGravity>(KineticModule::get_energy(
            self,
            *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ))
    }

    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController {
        std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(
            KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_CONTROL),
        )
    }

    unsafe fn handle_waveland(&mut self, require_airdodge: bool) -> bool {
        if require_airdodge && !self.is_status_one_of(&[*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE]) {
            return false;
        }

        if !crate::VarModule::is_flag(
            self.object(),
            crate::consts::vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET,
        ) {
            return false;
        }

        if self.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
            return false;
        }
    
        let pos = *PostureModule::pos(self);
        let upper_bound_offset_y = if StatusModule::is_changing(self) && !self.is_prev_status(*FIGHTER_STATUS_KIND_PASS) {
            crate::VarModule::get_float(self.object(), crate::consts::vars::common::instance::ECB_CENTER_Y_OFFSET)
        } else {
            crate::VarModule::get_float(self.object(), crate::consts::vars::common::instance::ECB_BOTTOM_Y_OFFSET)
        };
        let upper_bound_y = pos.y + upper_bound_offset_y;
        let snap_leniency = if WorkModule::get_float(self, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) <= 0.0 {
                upper_bound_offset_y
            } else {
                (upper_bound_offset_y).max(6.0)
            };
        let lower_bound = Vector2f::new(pos.x, upper_bound_y - snap_leniency);
        let ground_pos_any = &mut Vector2f::zero();
        let ground_pos_stage = &mut Vector2f::zero();
        let is_touch_any = GroundModule::line_segment_check(self, &Vector2f::new(pos.x, upper_bound_y), &lower_bound, &Vector2f::zero(), ground_pos_any, true);
        let is_touch_stage = GroundModule::line_segment_check(self, &Vector2f::new(pos.x, upper_bound_y), &lower_bound, &Vector2f::zero(), ground_pos_stage, false);
        let can_snap = !( 
            is_touch_any == 0 as *const *const u64
            || (is_touch_stage != 0 as *const *const u64
                && WorkModule::get_float(self, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) > 0.0)
        );
        if can_snap { // pretty sure it returns a pointer, at least it defo returns a non-0 value if success
            crate::VarModule::on_flag(self.object(), crate::consts::vars::common::status::DISABLE_ECB_SHIFT);
            PostureModule::set_pos(self, &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z));
            GroundModule::attach_ground(self, false);
            true
        } else {
            false
        }
    }

    unsafe fn status(&mut self) -> i32 {
        return StatusModule::status_kind(self);
    }

    unsafe fn lr(&mut self) -> f32 {
        return PostureModule::lr(self);
    }

    unsafe fn check_jump_cancel(&mut self, update_lr: bool) -> bool {
        let fighter = crate::util::get_fighter_common_from_accessor(self);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            WorkModule::enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
            );
            WorkModule::enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
            );
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() // buffered aerials
            || fighter.sub_transition_group_check_ground_jump().get_bool() // regular jumps
            {
                if update_lr {
                    PostureModule::set_stick_lr(self, 0.0);
                    PostureModule::update_rot_y_lr(self);
                }

                return true;
            }
        } else {
            WorkModule::enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
            );
            WorkModule::enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
            );
            WorkModule::enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY,
            );
            WorkModule::enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON,
            );
            WorkModule::enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT,
            );
            if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                return true;
            }
        }
        false
    }

    unsafe fn check_airdodge_cancel(&mut self) -> bool {
        let fighter = crate::util::get_fighter_common_from_accessor(self);
        WorkModule::enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR,
        );
        if fighter.sub_transition_group_check_air_escape().get_bool() {
            return true;
        }
        false
    }

    unsafe fn check_aerial_cancel(&mut self) -> bool {
        let fighter = crate::util::get_fighter_common_from_accessor(self);
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && fighter.get_aerial() != None {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
            return true;
        }
        return false;
    }

    unsafe fn check_dash_cancel(&mut self) -> bool {
        if self.is_situation(*SITUATION_KIND_GROUND) {
            if self.is_cat_flag(Cat1::Dash) {
                self.change_status_req(*FIGHTER_STATUS_KIND_DASH, false);
                return true;
            } else if self.is_cat_flag(Cat1::TurnDash) {
                self.change_status_req(*FIGHTER_STATUS_KIND_TURN_DASH, false);
                return true;
            }
        }
        false
    }

    unsafe fn check_wall_jump_cancel(&mut self) -> bool {
        if crate::VarModule::is_flag(self.object(), vars::common::instance::SPECIAL_WALL_JUMP) {
            return false;
        }
        crate::VarModule::on_flag(self.object(), vars::common::status::ENABLE_SPECIAL_WALLJUMP);
        let fighter = crate::util::get_fighter_common_from_accessor(self);
        if fighter.sub_transition_group_check_air_wall_jump().get_bool() {
            crate::VarModule::on_flag(self.object(), vars::common::instance::SPECIAL_WALL_JUMP);
            return true;
        }
        crate::VarModule::off_flag(self.object(), vars::common::status::ENABLE_SPECIAL_WALLJUMP);
        false
    }

    unsafe fn check_land_cancel(&mut self, landing_lag: Option<f32>) -> bool {
        if self.is_prev_situation(*SITUATION_KIND_AIR)
        && self.is_situation(*SITUATION_KIND_GROUND) {
            match landing_lag {
                Some(landing_lag) => {
                    VarModule::set_float(self.object(), vars::common::instance::LAND_CANCEL_LAG, landing_lag);
                },
                None => {}
            }

            StatusModule::change_status_request_from_script(self, *FIGHTER_STATUS_KIND_LANDING, false);

            return true;
        }

        false
    }

    unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x530 / 4) = x;
        *ground_data.add(0x534 / 4) = y;
    }

    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x540 / 4) = x;
        *ground_data.add(0x544 / 4) = y;
    }

    unsafe fn set_center_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x520 / 4) = x;
        *ground_data.add(0x524 / 4) = y;
    }

    unsafe fn get_front_cliff_hangdata(&mut self) -> Vector2f {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        let x = *ground_data.add(0x530 / 4);
        let y = *ground_data.add(0x534 / 4);
        Vector2f::new(x, y)
    }

    unsafe fn get_back_cliff_hangdata(&mut self) -> Vector2f {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        let x = *ground_data.add(0x540 / 4);
        let y = *ground_data.add(0x544 / 4);
        Vector2f::new(x, y)
    }

    unsafe fn get_center_cliff_hangdata(&mut self) -> Vector2f {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        let x = *ground_data.add(0x520 / 4);
        let y = *ground_data.add(0x524 / 4);
        Vector2f::new(x, y)
    }

    unsafe fn paradox_funcs(&mut self) {
        let fighter = crate::util::get_fighter_common_from_accessor(self);
        let death_statuses = &[
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_STANDBY,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY
        ];
        let damage_statuses = &[
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE
        ];
        let dive_cont_value = fighter.get_param_float("common", "dive_cont_value");
        let dive_flick_frame_value = fighter.get_param_int("common", "dive_flick_frame_value");

        if GroundModule::is_passable_ground(fighter.module_accessor)
        && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < 4
        && fighter.left_stick_y() < fighter.get_param_float("common", "pass_stick_y")
        && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_RUN,
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_GUARD_OFF
        ]) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
        }

        if !smashball::is_training_mode()
        && !lua_bind::FighterManager::is_result_mode(super::singletons::FighterManager())
        && sv_information::is_ready_go() 
        && Fighter::get_fighter_entry_count() != 0 {
            if fighter.is_status_one_of(death_statuses) 
            || fighter.is_status_one_of(damage_statuses) {
                VarModule::set_int(fighter.object(), vars::common::instance::STALL_TIMER, 0);
            } else {
                let num_players = Fighter::get_fighter_entry_count();
                let mut any_player_touched = false;
                for i in 0..num_players {
                    let opponent_boma = &mut *sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
                    if opponent_boma.is_status_one_of(death_statuses) 
                    || opponent_boma.is_status_one_of(damage_statuses) {
                        any_player_touched = true;
                    }
                }
                if any_player_touched {
                    VarModule::set_int(fighter.object(), vars::common::instance::STALL_TIMER, 0);
                } else {
                    VarModule::add_int(fighter.object(), vars::common::instance::STALL_TIMER, 1);
                }
            }
            if VarModule::get_int(fighter.object(), vars::common::instance::STALL_TIMER) >= 1200 {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_DEAD, false);
            }
        }

        if fighter.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
            if fighter.status_frame() < 1 {
                let angle = (KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) / (KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) + if KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0 { 0.00001 } else { 0.0 })).atan();
                if KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0. {
                    EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), &Vector3f {x: 0., y: 3., z: 0.}, &Vector3f{x: 0., y: 0., z: (90. + 180. * angle / 3.14159)}, 0.75, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, false, 0, 0, 0);
                } else {
                    EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), &Vector3f {x: 0., y: 3., z: 0.}, &Vector3f{x: 0., y: 0., z: (-90. + 180. * angle / 3.14159)}, 0.75, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, false, 0, 0, 0);
                }
            }

            if fighter.motion_frame() >= 5.0
            && !CancelModule::is_enable_cancel(fighter.module_accessor) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }

            if fighter.is_situation(*SITUATION_KIND_AIR) {
                fighter.sub_air_check_fall_common();
            }
        }

        if fighter.is_status(*FIGHTER_STATUS_KIND_LANDING)
        && fighter.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
        ]) {
            if (fighter.status_frame() as f32) < fighter.get_param_float("param_motion", "landing_frame_escape_air_slide_max") {
                let terms_to_enable = [
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
                ];
                fighter.enable_transition_term_many(&terms_to_enable);
                fighter.sub_transition_group_check_ground_item().get_bool();
                fighter.sub_transition_group_check_ground_special().get_bool();
                fighter.sub_transition_group_check_ground_attack().get_bool();
                fighter.sub_transition_group_check_ground_jump().get_bool();
            }
        }

        if fighter.is_status(*FIGHTER_STATUS_KIND_LANDING)
        && GroundModule::is_passable_ground(fighter.module_accessor)
        && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < 4
        && fighter.left_stick_y() < fighter.get_param_float("common", "pass_stick_y")
        && fighter.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
        ]) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
        }

        if VarModule::get_int(fighter.object(), vars::common::instance::CLIFF_XLU_FRAME) > 0 {
            VarModule::dec_int(fighter.object(), vars::common::instance::CLIFF_XLU_FRAME);
            if VarModule::get_int(fighter.object(), vars::common::instance::CLIFF_XLU_FRAME) - 1 == 0 
            || fighter.is_situation(*SITUATION_KIND_GROUND) {
                HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
                VarModule::set_int(fighter.object(), vars::common::instance::CLIFF_XLU_FRAME, 0);
            }
        }

        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 999999 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
        }

        if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < 4
        && fighter.left_stick_y() < fighter.get_param_float("common", "pass_stick_y") {
            GroundModule::clear_pass_floor(fighter.module_accessor);
        }

        if fighter.is_situation(*SITUATION_KIND_AIR)
        && (fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) 
        || (fighter.kind() == *FIGHTER_KIND_SAMUS && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW
        ])) || (fighter.kind() == *FIGHTER_KIND_FOX && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT
        ])) || (fighter.kind() == *FIGHTER_KIND_KOOPA && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S
        ])) || (fighter.kind() == *FIGHTER_KIND_MARIOD && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
        ])) || (fighter.kind() == *FIGHTER_KIND_FALCO && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            statuses::falco::SPECIAL_LW_LOOP,
            statuses::falco::SPECIAL_LW_HIT
        ])) || (fighter.kind() == *FIGHTER_KIND_METAKNIGHT && fighter.is_status_one_of(&[
            *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK
        ])) || (fighter.kind() == *FIGHTER_KIND_IKE && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP,
            *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END,
            *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL,
            *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD,
            *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT
        ])) || (fighter.kind() == *FIGHTER_KIND_LUCARIO && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW
        ])) || (fighter.kind() == *FIGHTER_KIND_WOLF && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT
        ])) || (fighter.kind() == *FIGHTER_KIND_RIDLEY && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH,
            statuses::ridley::SPECIAL_LW_POGO
        ]))) && !StatusModule::is_changing(fighter.module_accessor) {
            if fighter.left_stick_y() <= -0.6 && VarModule::get_int(fighter.object(), vars::common::instance::LEFT_STICK_FLICK_Y) < 4
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            } else {
                fighter.sub_air_check_dive();
            }
        }

        if fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR) {
            fighter.FighterStatusDamage__correctDamageVectorEffect(L2CValue::Bool(false));
        }

        if KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs() >= fighter.get_param_float("common", "invalid_passive_speed") 
        && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
        ]) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DEAD, false);
        }

        if fighter.is_situation(*SITUATION_KIND_AIR)
        && fighter.is_status_one_of(damage_statuses) 
        && !StopModule::is_stop(fighter.module_accessor) {
            let damage_speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let mut initial_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_X);
            let mut initial_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_Y);
            if initial_speed_x == 0.0 && initial_speed_y == 0.0 {
                VarModule::set_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_X, damage_speed_x);
                VarModule::set_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_Y, damage_speed_y);
                initial_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_X);
                initial_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::INITIAL_KNOCKBACK_VEL_Y);
            }
            let drift_value = fighter.left_stick_x() * (0.0075 *  (1.0 - (initial_speed_x.abs() / 3.0).clamp(0.0, 1.0)));
            fighter.set_speed(Vector2f::new(damage_speed_x + drift_value, damage_speed_y), *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        }

        if fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0
        && !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY])
        && fighter.is_button_trigger(Buttons::AppealAll)
        && !VarModule::is_flag(fighter.object(), vars::common::instance::BURST_LIMIT) {
            let lr = fighter.lr();
            let height = fighter.get_param_float("height", "");
            VarModule::on_flag(fighter.object(), vars::common::instance::BURST_LIMIT);
            smash::app::FighterUtil::flash_eye_info(fighter.module_accessor);
            if fighter.get_param_int("param_motion", "flip") != 0 {
                smash_script::macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, height, 2, 0, 0, 0, 0.66, true, *EF_FLIP_YZ);
            } else {
                smash_script::macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, height, 2, 0, 0, 0, 0.66, true);
            }
            smash_script::macros::LAST_EFFECT_SET_COLOR(fighter, 0.83, 0.69, 0.22);
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            fighter.set_float(0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
            if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
                fighter.dec_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
            CancelModule::enable_cancel(fighter.module_accessor);
        }

        if fighter.is_status_one_of(death_statuses) {
            VarModule::off_flag(fighter.object(), vars::common::instance::BURST_LIMIT);
        }
    }

    unsafe fn try_pickup_item(&mut self, range: f32, bone: Option<Hash40>, offset: Option<&Vector2f>) -> Option<&mut BattleObjectModuleAccessor> {
        use smash_rs::app::ItemManager;

        if !self.is_fighter() {
            return None;
        }

        let item_manager = ItemManager::instance().unwrap();

        if ItemModule::is_have_item(self, 0) {
            let have_id = ItemModule::get_have_item_id(self, 0);
            let item = item_manager.find_active_item_from_id(have_id as u32) as *mut smash::app::Item;
            let item_module_accessor = smash::app::lua_bind::Item::item_module_accessor(item) as *mut ItemModuleAccessor;
            let item_boma = &mut (*item_module_accessor).battle_object_module_accessor;
            return Some(item_boma);
        }
        
        let fighter_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let bone_hash = bone.unwrap_or(Hash40::new("top"));
        ModelModule::joint_global_position(self, bone_hash, fighter_pos, false);
        fighter_pos.z = 0.0;
        match offset {
            Some(offset) => {
                fighter_pos.x += offset.x * PostureModule::lr(self);
                fighter_pos.y += offset.y;
            },
            None => {}
        }
        
        let total = item_manager.get_num_of_active_item_all();
        for id in 0..total {
            // pointer to the item
            let item_ptr = item_manager.get_active_item(id as u64);
            if item_ptr.is_null() {
                continue;
            }

            let item = item_ptr as *mut smash::app::Item;
            let item_module_accessor = smash::app::lua_bind::Item::item_module_accessor(item) as *mut ItemModuleAccessor;
            let item_boma = &mut (*item_module_accessor).battle_object_module_accessor;
            let item_pos = PostureModule::pos(item_boma);

            if ((*item_pos).x - (*fighter_pos).x).abs() < range
                && ((*item_pos).y - (*fighter_pos).y).abs() < range {
                ItemModule::have_item_instance(self, item, 0, false, false, false, false);
                return Some(item_boma);
            }
        }
        return None;
    }

    unsafe fn get_player_idx_from_boma(&mut self) -> i32 {
        let control_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x48 / 8);
        let next = *((control_module + 0x118) as *const u64);
        let next = *((next + 0x58) as *const u64);
        let next = *((next + 0x8) as *const u64);
        *((next + 0x8) as *const i32)
    }

    unsafe fn set_command_input_button(&mut self, command: usize, buttons: u8) {
        let control_module = *(self as *mut BattleObjectModuleAccessor as *const *const u64).add(0x48 / 8);
        let command_input = *control_module.add((0x7f0 + (command * 8)) / 8) as *mut u8;
        *command_input.add(0xb) = buttons;
    }

    unsafe fn clone_command_input(&mut self, command: usize, replace_command: usize) {
        let control_module = *(self as *mut BattleObjectModuleAccessor as *const *const u64).add(0x48 / 8);
        let original = *control_module.add((0x7f0 + (command * 8)) / 8) as *mut CommandInputState;
        let replace = *control_module.add((0x7f0 + (replace_command * 8)) / 8) as *mut CommandInputState;
        *replace = *original.clone();
    }
}

pub trait LuaUtil {
    unsafe fn get_speed_x(&mut self, kinetic_id: i32) -> f32;
    unsafe fn get_speed_y(&mut self, kinetic_id: i32) -> f32;
    unsafe fn set_speed(&mut self, speed: Vector2f, kinetic_id: i32);
}

impl LuaUtil for L2CAgentBase {
    unsafe fn get_speed_x(&mut self, kinetic_id: i32) -> f32 {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id);
        app::sv_kinetic_energy::get_speed_x(self.lua_state_agent)
    }

    unsafe fn get_speed_y(&mut self, kinetic_id: i32) -> f32 {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id);
        app::sv_kinetic_energy::get_speed_y(self.lua_state_agent)
    }

    unsafe fn set_speed(&mut self, speed: Vector2f, kinetic_id: i32) {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id, speed.x, speed.y);
        app::sv_kinetic_energy::set_speed(self.lua_state_agent);
    }
}

pub trait GetObjects {
    unsafe fn boma(&mut self) -> &'static mut BattleObjectModuleAccessor {
        Self::get_boma(self)
    }

    unsafe fn object(&mut self) -> &'static mut BattleObject {
        Self::get_object(self)
    }

    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor;
    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject;
}

impl GetObjects for smash::lib::L2CAgent {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(this.battle_object)
    }
}

impl GetObjects for BattleObject {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(_: &mut Self) -> &'static mut BattleObject {
        panic!("Gannot call GetObjects::get_object on BattleObject!")
    }
}

impl GetObjects for BattleObjectModuleAccessor {
    unsafe fn get_boma(_: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        panic!("Gannot call GetObjects::get_boma on BattleObjectModuleAccessor!")
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(super::util::get_battle_object_from_id(
            this.battle_object_id,
        ))
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputKind {
    Attack = 0x0,
    Special = 0x1,
    Jump = 0x2,
    Guard = 0x3,
    Grab = 0x4,
    SmashAttack = 0x5,
    AppealHi = 0xA,
    AppealS = 0xB,
    AppealLw = 0xC,
    Unset = 0xD,
    JumpMini = 0x12,   
    TiltAttack = 0x13, 
    Parry = 0x14,     
}

#[derive(Debug)]
#[repr(C)]
pub struct ControllerMapping {
    pub gc_l: InputKind,
    pub gc_r: InputKind,
    pub gc_z: InputKind,
    pub gc_dup: InputKind,
    pub gc_dlr: InputKind,
    pub gc_ddown: InputKind,
    pub gc_a: InputKind,
    pub gc_b: InputKind,
    pub gc_cstick: InputKind,
    pub gc_y: InputKind,
    pub gc_x: InputKind,
    pub gc_rumble: bool,
    pub gc_absmash: u8,
    pub gc_tapjump: bool,
    pub gc_sensitivity: u8,
    // 0xF
    pub pro_l: InputKind,
    pub pro_r: InputKind,
    pub pro_zl: InputKind,
    pub pro_zr: InputKind,
    pub pro_dup: InputKind,
    pub pro_dlr: InputKind,
    pub pro_ddown: InputKind,
    pub pro_a: InputKind,
    pub pro_b: InputKind,
    pub pro_cstick: InputKind,
    pub pro_x: InputKind,
    pub pro_y: InputKind,
    pub pro_rumble: bool,
    pub pro_absmash: u8,
    pub pro_tapjump: bool,
    pub pro_sensitivity: u8,
    // 0x1F
    pub joy_shoulder: InputKind,
    pub joy_zshoulder: InputKind,
    pub joy_sl: InputKind,
    pub joy_sr: InputKind,
    pub joy_up: InputKind,
    pub joy_right: InputKind,
    pub joy_left: InputKind,
    pub joy_down: InputKind,
    pub joy_rumble: bool,
    pub joy_absmash: u8,
    pub joy_tapjump: bool,
    pub joy_sensitivity: u8,
    // 0x2B
    pub _2b: u8,
    pub _2c: u8,
    pub _2d: u8,
    pub _2e: u8,
    pub _2f: u8,
    pub _30: u8,
    pub _31: u8,
    pub _32: u8,
    pub is_absmash: bool,
    pub _34: [u8; 0x1C],
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Controller {
    pub vtable: *const u64,
    pub current_buttons: ButtonBitfield,
    pub previous_buttons: ButtonBitfield,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub left_trigger: f32,
    pub _left_padding: u32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
    pub right_trigger: f32,
    pub _right_padding: u32,
    pub gyro: [f32; 4],
    pub button_timespan: AutorepeatInfo,
    pub lstick_timespan: AutorepeatInfo,
    pub rstick_timespan: AutorepeatInfo,
    pub just_down: ButtonBitfield,
    pub just_release: ButtonBitfield,
    pub autorepeat_keys: u32,
    pub autorepeat_threshold: u32,
    pub autorepeat_initial_press_threshold: u32,
    pub style: ControllerStyle,
    pub controller_id: u32,
    pub primary_controller_color1: u32,
    pub primary_controller_color2: u32,
    pub secondary_controller_color1: u32,
    pub secondary_controller_color2: u32,
    pub led_pattern: u8,
    pub button_autorepeat_initial_press: bool,
    pub lstick_autorepeat_initial_press: bool,
    pub rstick_autorepeat_initial_press: bool,
    pub is_valid_controller: bool,
    pub _xB9: [u8; 2],
    pub is_connected: bool,
    pub is_left_connected: bool,
    pub is_right_connected: bool,
    pub is_wired: bool,
    pub is_left_wired: bool,
    pub is_right_wired: bool,
    pub _xC1: [u8; 3],
    pub npad_number: u32,
    pub _xC8: [u8; 8],
}

/// Re-ordered bitfield the game uses for buttons
#[bitfield]
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct ButtonBitfield {
    pub dpad_up: bool,
    pub dpad_right: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub x: bool,
    pub a: bool,
    pub b: bool,
    pub y: bool,
    pub l: bool,
    pub r: bool,
    pub zl: bool,
    pub zr: bool,
    pub left_sl: bool,
    pub left_sr: bool,
    pub right_sl: bool,
    pub right_sr: bool,
    pub stick_l: bool,
    pub stick_r: bool,
    pub plus: bool,
    pub minus: bool,
    pub l_up: bool,
    pub l_right: bool,
    pub l_down: bool,
    pub l_left: bool,
    pub r_up: bool,
    pub r_right: bool,
    pub r_down: bool,
    pub r_left: bool,
    pub real_digital_l: bool,
    pub real_digital_r: bool,
    pub unused: B2,
}

#[repr(C)]
pub struct AutorepeatInfo {
    field: [u8; 0x18],
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum ControllerStyle {
    Handheld = 0x1,
    DualJoycon = 0x2,
    LeftJoycon = 0x3,
    RightJoycon = 0x4,
    ProController = 0x5,
    DebugPag = 0x6, // I assume
    GCController = 0x7,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MappedInputs {
    pub buttons: Buttons,
    pub lstick_x: i8,
    pub lstick_y: i8,
    pub rstick_x: i8,
    pub rstick_y: i8,
}

#[repr(C)]
pub struct CollisionLog {
    pub next: *mut CollisionLog,
    pub end: *mut CollisionLog,
    pub location: Vector3f,
    pub padding_0: u32,
    pub padding_1: u32,
    pub opponent_battle_object_id: u32,
    pub padding_2: [u8;7],
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
    pub padding_3: [u8;10]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandInputState {
    pub vtable: u64,
    pub command_timer: u8,
    pub state: u8,
    pub unk2: u8,
    pub input_allow: u8,
    pub max_timer: u8,
    pub enable_timer: u8,
    pub lr: i8,
}