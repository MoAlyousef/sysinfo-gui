use crate::{
    gui::{message::Message, View},
    widgets::{Card, HollowRoundToggle, RoundToggle, Toggle},
};
use fltk::{enums::*, prelude::*, *};

pub fn settings() -> group::Pack {
    let mut grp = group::Pack::default().with_size(80, 400).center_of_parent();
    grp.set_spacing(40);
    RoundToggle::new(300, 300, 60, 30);
    HollowRoundToggle::new(300, 300, 60, 34);
    Toggle::new(300, 300, 60, 15);
    grp.end();
    grp
}
