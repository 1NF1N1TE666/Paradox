use super::*;

mod acmd;

pub fn install() {
    let popo = &mut Agent::new("popo_wave");
    let nana = &mut Agent::new("nana_wave");

    acmd::install(popo);
    acmd::install(nana);

    popo.install();
    nana.install();
}