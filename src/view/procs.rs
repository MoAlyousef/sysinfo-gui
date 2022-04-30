use crate::{
    gui::{message::Message, View},
    logic::{message::SysMsg, CHAN, SLEEP},
    widgets::{Card, HollowRoundToggle, RoundToggle, Toggle},
};
use fltk::{prelude::*, enums::*, *};

pub fn procs() -> group::Pack {
    let mut grp = group::Pack::default()
        .with_size(80, 400)
        .center_of_parent();
    grp.set_spacing(40);
    RoundToggle::new(300, 300, 60, 30);
    HollowRoundToggle::new(300, 300, 60, 30);
    Toggle::new(300, 300, 60, 15);
    grp.end();
    grp
}
