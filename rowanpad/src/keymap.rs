use rmk::action::KeyAction;
use rmk::{k, layer, mo};
pub(crate) const COL: usize = 2;
pub(crate) const ROW: usize = 2;
pub(crate) const NUM_LAYER: usize = 2;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [mo!(1), k!(B)], [k!(Kp4), k!(LShift)]
        ]),
        layer!([
            [k!(Kp7), k!(Kp8)], [k!(Kp4), k!(LCtrl)]
        ]),
    ]
}
