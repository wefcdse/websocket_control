use crate::{Errors, ToErrorsResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}
impl Side {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Side::Top => "top",
            Side::Bottom => "bottom",
            Side::Left => "left",
            Side::Right => "right",
            Side::Front => "front",
            Side::Back => "back",
        }
    }
}

impl TryFrom<&str> for Side {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "top" => Self::Top,
            "bottom" => Self::Bottom,
            "left" => Self::Left,
            "right" => Self::Right,
            "front" => Self::Front,
            "back" => Self::Back,
            _ => Err(Errors::InvalidSideName(value.to_owned()))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event {
    Key { keycode: u16, hold: bool },
    KeyUp { keycode: u16 },
    Other(String),
    MouseClick { key: u8, x: u16, y: u16 },
    MouseUp { key: u8, x: u16, y: u16 },
    MouseDrag { key: u8, x: u16, y: u16 },
    MouseScroll { direction: i8, x: u16, y: u16 },
    MonitorTouch { side: Side, x: u16, y: u16 },
}

impl TryFrom<String> for Event {
    type Error = Errors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let sp = value.split(" ").collect::<Vec<_>>();
        let evt_name = sp.get(0).to_errors_result()?;
        Ok(match *evt_name {
            "key" => {
                let keycode = sp.get(1).to_errors_result()?.parse()?;
                let hold = sp.get(2).to_errors_result()?.parse()?;
                Self::Key { keycode, hold }
            }
            "key_up" => {
                let keycode = sp.get(1).to_errors_result()?.parse()?;
                Self::KeyUp { keycode }
            }
            "mouse_click" => {
                let key = sp.get(1).to_errors_result()?.parse()?;
                let x = sp.get(2).to_errors_result()?.parse()?;
                let y = sp.get(3).to_errors_result()?.parse()?;
                Self::MouseClick { key, x, y }
            }
            "mouse_up" => {
                let key = sp.get(1).to_errors_result()?.parse()?;
                let x = sp.get(2).to_errors_result()?.parse()?;
                let y = sp.get(3).to_errors_result()?.parse()?;
                Self::MouseUp { key, x, y }
            }
            "mouse_drag" => {
                let key = sp.get(1).to_errors_result()?.parse()?;
                let x = sp.get(2).to_errors_result()?.parse()?;
                let y = sp.get(3).to_errors_result()?.parse()?;
                Self::MouseDrag { key, x, y }
            }
            "mouse_scroll" => {
                let direction = sp.get(1).to_errors_result()?.parse()?;
                let x = sp.get(2).to_errors_result()?.parse()?;
                let y = sp.get(3).to_errors_result()?.parse()?;
                Self::MouseScroll { direction, x, y }
            }
            "monitor_touch" => {
                let side = (*sp.get(1).to_errors_result()?).try_into()?;
                let x = sp.get(2).to_errors_result()?.parse()?;
                let y = sp.get(3).to_errors_result()?.parse()?;
                Self::MonitorTouch { side, x, y }
            }
            _ => Self::Other(value),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PeripheralType {
    Monitor,
    Modem,
    Speaker,
    Drive,
    Printer,
    Inventory,
}

impl PeripheralType {
    pub fn name(&self) -> &'static str {
        match self {
            PeripheralType::Monitor => "Monitor",
            PeripheralType::Modem => "modem",
            PeripheralType::Speaker => "speaker",
            PeripheralType::Drive => "drive",
            PeripheralType::Printer => "printer",
            PeripheralType::Inventory => "inventory",
        }
    }
}

impl TryFrom<&str> for PeripheralType {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "monitor" => Self::Monitor,
            "modem" => Self::Modem,
            "speaker" => Self::Speaker,
            "drive" => Self::Drive,
            "printer" => Self::Printer,
            "inventory" => Self::Inventory,
            _ => Err(Errors::InvalidPeripheralType(value.to_owned()))?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorId {
    White = 0,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}
impl ColorId {
    pub fn to_number(&self) -> u16 {
        1 << *self as u8
    }

    pub fn from_number_overflow(num: u32) -> ColorId {
        let num = num % 16;
        let num = num as u8;
        ColorId::from_number_or_panic(num)
    }

    fn from_number_or_panic(num: u8) -> ColorId {
        match num {
            0 => ColorId::White,
            1 => ColorId::Orange,
            2 => ColorId::Magenta,
            3 => ColorId::LightBlue,
            4 => ColorId::Yellow,
            5 => ColorId::Lime,
            6 => ColorId::Pink,
            7 => ColorId::Gray,
            8 => ColorId::LightGray,
            9 => ColorId::Cyan,
            10 => ColorId::Purple,
            11 => ColorId::Blue,
            12 => ColorId::Brown,
            13 => ColorId::Green,
            14 => ColorId::Red,
            15 => ColorId::Black,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    PosX,
    PosY,
    NegX,
    NegY,
}
impl Direction {
    pub fn to_dxdy(&self) -> (isize, isize) {
        match self {
            Direction::PosX => (1, 0),
            Direction::PosY => (0, 1),
            Direction::NegX => (-1, 0),
            Direction::NegY => (0, -1),
        }
    }
}
