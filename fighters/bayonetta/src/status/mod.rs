use super::*;

mod attack;
mod batwithin;
mod escape;
mod attackair;
mod specialairs;
mod specialn;
mod specials;
mod jumpaerial;

pub fn install() {
    attack::install();
    batwithin::install();
    escape::install();
    attackair::install();
    specialairs::install();
    specialn::install();
    specials::install();
    jumpaerial::install();
}