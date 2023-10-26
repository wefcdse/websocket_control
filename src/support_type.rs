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
    C01 = 0,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
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
            0 => ColorId::C01,
            1 => ColorId::C02,
            2 => ColorId::C03,
            3 => ColorId::C04,
            4 => ColorId::C05,
            5 => ColorId::C06,
            6 => ColorId::C07,
            7 => ColorId::C08,
            8 => ColorId::C09,
            9 => ColorId::C10,
            10 => ColorId::C11,
            11 => ColorId::C12,
            12 => ColorId::C13,
            13 => ColorId::C14,
            14 => ColorId::C15,
            15 => ColorId::C16,
            _ => panic!(),
        }
    }
}
