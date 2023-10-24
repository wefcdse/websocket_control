use crate::{Errors, Port, Side};

// redstone
impl<'a> Port<'a> {
    pub async fn get_redstone(&mut self, side: Side) -> Result<i32, Errors> {
        self.send(format!("g_rs {}", side.name())).await?;
        let received = self.receive().await?;
        let num: f32 = received.parse()?;
        let num = num as i32;
        Ok(num)
    }
    pub async fn set_redstone(&mut self, side: Side, level: i32) -> Result<(), Errors> {
        if !(0..=15).contains(&level) {
            return Err(Errors::InvalidRedstoneLevel(level));
        }
        self.send(format!("s_rs {} {}", side.name(), level)).await?;
        Ok(())
    }
}
