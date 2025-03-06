use rmk::action::KeyAction;
use rmk::{a, k, layer, mo, osl};
pub(crate) const COL: usize = 12;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 2;
#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    //          |  1  |  2  | 3 | 4 | 5  |        | 6  | 7  | 8  | 9  | 0 |
    // | GRAVE  |  q  |  w  | e | r | t  |        | y  | u  | i  | o  | p | BKSPC |
    // |   Tab  |  a  |  s  | d | f | g  |        | h  | j  | k  | l  | ; | ENTER |
    // | LShift |  z  |  x  | c | v | b  |        | n  | m  | ,  | .  | / |   \   |
    // | LCtrl  |  [  |  ]  | < | ^ | L1 |        | L1 | V | > | " | CAPS |  DEL  |
    //          | ESC | EQUAL | LGUI  | ENTER | | SPACE | RGUI | MINUS | PRTSC |
    [
        layer!([
            [a!(No),     k!(Kc1),         k!(Kc2),          k!(Kc3),   k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(Minus), k!(Equal), k!(Backspace)],
            [k!(Grave),  k!(Q),           k!(W),            k!(E),     k!(R),   k!(T), k!(Y), k!(U), k!(I), k!(O), k!(P), k!(LeftBracket), k!(RightBracket), k!(Backslash)],
            [k!(Tab),    k!(A),           k!(S),            k!(D),     k!(F),   k!(G), k!(H), k!(J), k!(K), k!(L), k!(Semicolon), k!(Quote), a!(No), k!(Enter)],
            [k!(LShift), k!(Z),           k!(X),            k!(C),     k!(V),   k!(B), k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash), a!(No), a!(No), k!(RShift)],
            [k!(LCtrl),  k!(LeftBracket), k!(RightBracket), k!(Left),  k!(UP),  osl!(1), a!(No), a!(No), a!(No), mo!(1), k!(RAlt), a!(No), k!(RGui), k!(RCtrl)],
            [k!(Escape), k!(Equal),       k!(LGui),         k!(Enter), a!(No),  a!(No), a!(No), a!(No), mo!(1), k!(RAlt), a!(No), k!(RGui), k!(RCtrl)]
        ]),
        layer!([
            [k!(Grave), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10), k!(F11), k!(F12), k!(Delete)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(CapsLock), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(UP)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(Left), a!(No), k!(Down), k!(Right)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(Left), a!(No), k!(Down), k!(Right)],
        ]),
    ]
}
