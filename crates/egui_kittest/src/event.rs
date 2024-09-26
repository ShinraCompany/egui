use kittest::{Key, MouseButton};

pub fn kittest_key_to_egui(value: Key) -> Option<egui::Key> {
    match value {
        Key::ArrowDown => Some(egui::Key::ArrowDown),
        Key::ArrowLeft => Some(egui::Key::ArrowLeft),
        Key::ArrowRight => Some(egui::Key::ArrowRight),
        Key::ArrowUp => Some(egui::Key::ArrowUp),
        Key::Escape => Some(egui::Key::Escape),
        Key::Tab => Some(egui::Key::Tab),
        Key::Backspace => Some(egui::Key::Backspace),
        Key::Enter => Some(egui::Key::Enter),
        Key::Space => Some(egui::Key::Space),
        Key::Insert => Some(egui::Key::Insert),
        Key::Delete => Some(egui::Key::Delete),
        Key::Home => Some(egui::Key::Home),
        Key::End => Some(egui::Key::End),
        Key::PageUp => Some(egui::Key::PageUp),
        Key::PageDown => Some(egui::Key::PageDown),
        Key::Copy => Some(egui::Key::Copy),
        Key::Cut => Some(egui::Key::Cut),
        Key::Paste => Some(egui::Key::Paste),
        Key::Colon => Some(egui::Key::Colon),
        Key::Comma => Some(egui::Key::Comma),
        Key::Backslash => Some(egui::Key::Backslash),
        Key::Slash => Some(egui::Key::Slash),
        Key::Pipe => Some(egui::Key::Pipe),
        Key::Questionmark => Some(egui::Key::Questionmark),
        Key::OpenBracket => Some(egui::Key::OpenBracket),
        Key::CloseBracket => Some(egui::Key::CloseBracket),
        Key::Backtick => Some(egui::Key::Backtick),
        Key::Minus => Some(egui::Key::Minus),
        Key::Period => Some(egui::Key::Period),
        Key::Plus => Some(egui::Key::Plus),
        Key::Equals => Some(egui::Key::Equals),
        Key::Semicolon => Some(egui::Key::Semicolon),
        Key::Quote => Some(egui::Key::Quote),
        Key::Num0 => Some(egui::Key::Num0),
        Key::Num1 => Some(egui::Key::Num1),
        Key::Num2 => Some(egui::Key::Num2),
        Key::Num3 => Some(egui::Key::Num3),
        Key::Num4 => Some(egui::Key::Num4),
        Key::Num5 => Some(egui::Key::Num5),
        Key::Num6 => Some(egui::Key::Num6),
        Key::Num7 => Some(egui::Key::Num7),
        Key::Num8 => Some(egui::Key::Num8),
        Key::Num9 => Some(egui::Key::Num9),
        Key::A => Some(egui::Key::A),
        Key::B => Some(egui::Key::B),
        Key::C => Some(egui::Key::C),
        Key::D => Some(egui::Key::D),
        Key::E => Some(egui::Key::E),
        Key::F => Some(egui::Key::F),
        Key::G => Some(egui::Key::G),
        Key::H => Some(egui::Key::H),
        Key::I => Some(egui::Key::I),
        Key::J => Some(egui::Key::J),
        Key::K => Some(egui::Key::K),
        Key::L => Some(egui::Key::L),
        Key::M => Some(egui::Key::M),
        Key::N => Some(egui::Key::N),
        Key::O => Some(egui::Key::O),
        Key::P => Some(egui::Key::P),
        Key::Q => Some(egui::Key::Q),
        Key::R => Some(egui::Key::R),
        Key::S => Some(egui::Key::S),
        Key::T => Some(egui::Key::T),
        Key::U => Some(egui::Key::U),
        Key::V => Some(egui::Key::V),
        Key::W => Some(egui::Key::W),
        Key::X => Some(egui::Key::X),
        Key::Y => Some(egui::Key::Y),
        Key::Z => Some(egui::Key::Z),
        Key::F1 => Some(egui::Key::F1),
        Key::F2 => Some(egui::Key::F2),
        Key::F3 => Some(egui::Key::F3),
        Key::F4 => Some(egui::Key::F4),
        Key::F5 => Some(egui::Key::F5),
        Key::F6 => Some(egui::Key::F6),
        Key::F7 => Some(egui::Key::F7),
        Key::F8 => Some(egui::Key::F8),
        Key::F9 => Some(egui::Key::F9),
        Key::F10 => Some(egui::Key::F10),
        Key::F11 => Some(egui::Key::F11),
        Key::F12 => Some(egui::Key::F12),
        Key::F13 => Some(egui::Key::F13),
        Key::F14 => Some(egui::Key::F14),
        Key::F15 => Some(egui::Key::F15),
        Key::F16 => Some(egui::Key::F16),
        Key::F17 => Some(egui::Key::F17),
        Key::F18 => Some(egui::Key::F18),
        Key::F19 => Some(egui::Key::F19),
        Key::F20 => Some(egui::Key::F20),
        Key::F21 => Some(egui::Key::F21),
        Key::F22 => Some(egui::Key::F22),
        Key::F23 => Some(egui::Key::F23),
        Key::F24 => Some(egui::Key::F24),
        Key::F25 => Some(egui::Key::F25),
        Key::F26 => Some(egui::Key::F26),
        Key::F27 => Some(egui::Key::F27),
        Key::F28 => Some(egui::Key::F28),
        Key::F29 => Some(egui::Key::F29),
        Key::F30 => Some(egui::Key::F30),
        Key::F31 => Some(egui::Key::F31),
        Key::F32 => Some(egui::Key::F32),
        Key::F33 => Some(egui::Key::F33),
        Key::F34 => Some(egui::Key::F34),
        Key::F35 => Some(egui::Key::F35),
        _ => None,
    }
}

pub fn pointer_button_to_egui(value: MouseButton) -> Option<egui::PointerButton> {
    match value {
        MouseButton::Left => Some(egui::PointerButton::Primary),
        MouseButton::Right => Some(egui::PointerButton::Secondary),
        MouseButton::Middle => Some(egui::PointerButton::Middle),
        MouseButton::Back => Some(egui::PointerButton::Extra1),
        MouseButton::Forward => Some(egui::PointerButton::Extra2),
        MouseButton::Other(_) => None,
    }
}
