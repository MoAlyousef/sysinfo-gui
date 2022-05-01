use crate::widgets::{Card, FancyHorSlider, HollowRoundToggle, RoundToggle, Toggle};
use fltk::{enums::*, prelude::*, *};

pub fn settings() -> group::Pack {
    let mut grp = group::Pack::default().with_size(80, 400).center_of_parent();
    grp.set_spacing(40);
    RoundToggle::new(300, 300, 60, 30);
    HollowRoundToggle::new(300, 300, 60, 34);
    Toggle::new(300, 300, 60, 15);
    let mut f = FancyHorSlider::new(200, 200, 200, 10);
    f.set_callback(|_| println!("Works"));
    grp.end();
    grp
}
