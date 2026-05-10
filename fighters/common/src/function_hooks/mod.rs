use super::*;
use crate::globals::*;

pub mod attack_and_knockback;
pub mod collision;
pub mod energy;
pub mod iceclimber;
pub mod jumps;
pub mod kinetic;
pub mod momentum_transfer;
pub mod params;
pub mod situation;
pub mod transition;

mod lua_bind_hook;

pub fn install() {
    attack_and_knockback::install();
    collision::install();
    energy::install();
    iceclimber::install();
    jumps::install();
    kinetic::install();
    momentum_transfer::install();
    params::install();
    situation::install();
    transition::install();
    lua_bind_hook::install();

    unsafe {
        skyline::patching::Patch::in_text(0x6d21a8).data(0x52800015u32);
        skyline::patching::Patch::in_text(0x4ceae8).data(0x140000D1u32);
        skyline::patching::Patch::in_text(0x3fd360).nop();
        skyline::patching::Patch::in_text(0x614fb4).data(0x54000060);
    }
}
