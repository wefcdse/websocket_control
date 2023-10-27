use crate::{
    error::GpsError, utils::AsIfPixel, ColorId, Errors, Event, PeripheralType, Port, Side,
    ToErrorsResult,
};

// redstone
impl<'a> Port<'a> {
    /// get the redstone input signal strength for a specific side.
    pub async fn get_redstone(&mut self, side: Side) -> Result<i32, Errors> {
        self.send(format!("g_rs {}", side.name())).await?;
        let received = self.receive().await?;
        let num: f32 = received.parse()?;
        let num = num as i32;
        Ok(num)
    }

    /// set the redstone input signal strength for a specific side.
    pub async fn set_redstone(&mut self, side: Side, level: i32) -> Result<(), Errors> {
        if !(0..=15).contains(&level) {
            return Err(Errors::InvalidRedstoneLevel(level));
        }
        self.send(format!("s_rs {} {}", side.name(), level)).await?;
        Ok(())
    }
}

// gps
impl<'a> Port<'a> {
    /// retrieve the computer or turtles own location.
    pub async fn gps_locate(&mut self) -> Result<(f32, f32, f32), Errors> {
        self.send("gps_loc".to_owned()).await?;
        let received = self.receive().await?;
        if received == "failed" {
            return Err(Errors::GPSError(GpsError::Failed));
        }
        let received = received.split(' ').collect::<Vec<_>>();
        if received.len() != 3 {
            return Err(Errors::GPSError(GpsError::Other));
        }
        let x = received[0].parse()?;
        let y = received[1].parse()?;
        let z = received[2].parse()?;
        Ok((x, y, z))
    }
}

// event
impl<'a> Port<'a> {
    /// pull a event from the computer
    pub async fn pull_event(&mut self) -> Result<Option<Event>, Errors> {
        self.send("evt".to_owned()).await?;
        let msg = self.receive().await?;
        if msg == "none" {
            return Ok(None);
        }
        Ok(Some(Event::try_from(msg)?))
    }
}

// peripheral
impl<'a> Port<'a> {
    /// get the peripheral type for a specific side
    pub async fn get_peripheral(&mut self, side: Side) -> Result<Option<PeripheralType>, Errors> {
        self.send(format!("g_peri {}", side.name())).await?;
        let recv = self.receive().await?;
        Ok(if recv == "none" {
            None
        } else {
            Some(PeripheralType::try_from(&recv as &str)?)
        })
    }

    #[allow(unused)]
    async fn get_peripheral_str(&mut self, side: Side) -> Result<String, Errors> {
        self.send(format!("g_peri {}", side.name())).await?;
        let recv = self.receive().await?;
        Ok(recv)
    }
}

// monitor
impl<'a> Port<'a> {
    /// write a string to a monitor,
    /// at x, y,
    /// with the specific background and text color
    pub async fn monitor_write_string(
        &mut self,
        side: Side,
        x: usize,
        y: usize,
        background_color: ColorId,
        text_color: ColorId,
        text: &str,
    ) -> Result<(), Errors> {
        self.send(format!(
            "m_w_at_c {side} {x} {y} {c1} {c2}",
            side = side.name(),
            c1 = background_color.to_number(),
            c2 = text_color.to_number(),
        ))
        .await?;
        self.send(text.to_owned()).await?;
        Ok(())
    }

    /// write a `char` to a monitor,
    /// at x, y,
    /// with the specific background and text color
    /// the `char`should be ascii
    pub async fn monitor_write(
        &mut self,
        side: Side,
        x: usize,
        y: usize,
        background_color: ColorId,
        text_color: ColorId,
        text: char,
    ) -> Result<(), Errors> {
        if !text.is_ascii() {
            return Err(Errors::InvalidChar(text));
        }
        self.send(format!(
            "m_w_at_c_sig {side} {x} {y} {c1} {c2} {is_space} {text}",
            side = side.name(),
            c1 = background_color.to_number(),
            c2 = text_color.to_number(),
            is_space = text == ' ',
            text = if text == ' ' { '_' } else { text }
        ))
        .await?;
        Ok(())
    }

    pub async fn monitor_write_multi(
        &mut self,
        side: Side,
        pixels: &[(usize, usize, AsIfPixel)],
    ) -> Result<(), Errors> {
        let mut s = String::new();
        for (x, y, pixel) in pixels {
            let cmd = format!(
                " {x} {y} {c1} {c2} {is_space} {text}",
                c1 = pixel.background_color.to_number(),
                c2 = pixel.text_color.to_number(),
                is_space = pixel.text() == ' ',
                text = if pixel.text() == ' ' {
                    '_'
                } else {
                    pixel.text()
                }
            );
            s += &cmd;
        }
        self.send(format!(
            "msm {count} {side}{s}",
            count = pixels.len(),
            side = side.name(),
        ))
        .await?;
        Ok(())
    }

    /// get the size of a monitor
    pub async fn monitor_get_size(&mut self, side: Side) -> Result<Option<(usize, usize)>, Errors> {
        self.send(format!("m_g_sz {}", side.name())).await?;
        let recv = self.receive().await?;
        if recv == "none" {
            return Ok(None);
        }
        let sp = recv.split(' ').collect::<Vec<_>>();
        let x = sp.get(0).to_errors_result()?.parse()?;
        let y = sp.get(1).to_errors_result()?.parse()?;
        Ok(Some((x, y)))
    }
}

#[test]
fn special_chars() {
    for d in 0..127 {
        let c = char::from_u32(d).unwrap();
        let str = c.escape_default();
        println!("{} {} {}", d, c, str);
    }
}
