pub use vec2d::{IterIndex, Vec2d};
mod vec2d {
    use std::{
        fmt::{Debug, Display},
        ops::{Index, IndexMut},
    };

    /// a Vec2d collection which can be used in monitor.
    ///
    /// it stores the items in a `Vec<T>`, rather than `Vec<Vec<T>>`
    /// to avoid too many times of heap allowcation.
    ///
    /// ```
    /// use computercraft_websocket_control::utils::Vec2d;
    /// let mut v = Vec2d::new_filled_copy(2, 3, 0);
    /// v[(0, 1)] = 2;
    /// assert_eq!(v[0][1], 2);
    ///
    /// ```
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Vec2d<T> {
        inner: Vec<T>,
        x: usize,
        y: usize,
    }

    impl<T> Vec2d<T> {
        pub fn size(&self) -> (usize, usize) {
            (self.x, self.y)
        }
        pub fn x(&self) -> usize {
            self.x
        }
        pub fn y(&self) -> usize {
            self.y
        }
    }
    impl<'a, T> Index<usize> for Vec2d<T> {
        type Output = [T];

        fn index(&self, index: usize) -> &Self::Output {
            if !index < self.x {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.x, index
                );
            }
            &self.inner[index * self.y..(index + 1) * self.y]
        }
    }

    impl<'a, T> IndexMut<usize> for Vec2d<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if !index < self.x {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.x, index
                );
            }
            &mut self.inner[index * self.y..(index + 1) * self.y]
        }
    }

    impl<T: Debug> Debug for Vec2d<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut dbg_list = f.debug_list();
            for x in 0..self.x {
                dbg_list.entry(&(&self.inner[x * self.y..(x + 1) * self.y]) as &dyn Debug);
            }
            dbg_list.finish()
        }
    }

    impl<T: Debug> Display for Vec2d<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in 0..self.y {
                for x in 0..self.x {
                    write!(f, "{:?}\t", self[(x, y)])?;
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }
    #[test]
    fn test_display() {
        let mut v = Vec2d::new_filled_copy(2, 3, 0);
        v[(0, 1)] = 2;
        println!("{}", v);
    }

    impl<T: Clone> Vec2d<T> {
        pub fn new_filled(x: usize, y: usize, value: T) -> Self {
            Self {
                inner: {
                    let mut v = Vec::with_capacity(x * y);
                    for _ in 0..x * y {
                        v.push(value.clone());
                    }
                    v
                },
                x,
                y,
            }
        }
    }
    impl<T: Copy> Vec2d<T> {
        #[inline(always)]
        pub fn new_filled_copy(x: usize, y: usize, value: T) -> Self {
            Self {
                inner: vec![value; x * y],
                x,
                y,
            }
        }
    }
    impl<T> Index<(usize, usize)> for Vec2d<T> {
        type Output = T;

        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            &self.inner[x * self.y + y]
        }
    }

    impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
        fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
            &mut self.inner[x * self.y + y]
        }
    }

    impl<T> Vec2d<T> {
        pub fn into_iter(self) -> impl Iterator<Item = ((usize, usize), T)> {
            let y = self.y;
            self.inner
                .into_iter()
                .enumerate()
                .map(move |(idx, value)| ((idx / y, idx % y), value))
        }

        pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
            let y = self.y;
            self.inner
                .iter()
                .enumerate()
                .map(move |(idx, value)| ((idx / y, idx % y), value))
        }

        pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
            let y = self.y;
            self.inner
                .iter_mut()
                .enumerate()
                .map(move |(idx, value)| ((idx / y, idx % y), value))
        }

        pub fn iter_index(&self) -> IterIndex {
            IterIndex {
                now: 0,
                len: self.inner.len(),
                y: self.y(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct IterIndex {
        now: usize,
        len: usize,
        y: usize,
    }
    impl Iterator for IterIndex {
        type Item = (usize, usize);

        fn next(&mut self) -> Option<Self::Item> {
            if self.now < self.len {
                let idx = self.now;
                self.now += 1;
                Some((idx / self.y, idx % self.y))
            } else {
                None
            }
        }
    }

    #[test]
    fn a() {
        let mut v = Vec2d::new_filled_copy(2, 3, 0);
        v[(0, 1)] = 2;
        v[(1, 2)] = 20;
        v[1][1] = 11;
        dbg!(&v.inner[0..4]);
        dbg!(&v);
        println!("{:?}", v);
        println!("{}", v);
        for i in v.iter() {
            println!("{:?}", i);
        }
    }
}

pub use local_monitor::{AsIfPixel, LocalMonitor};

pub use save_lua_scripts::save_lua_scripts;
mod save_lua_scripts {
    use std::{fs, io::Write};
    /// this function can save the client side lua script to a specific file
    ///
    /// unzip the file and place the contents to computer craft's computer's
    /// script folder
    pub fn save_lua_scripts(path: &str) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .unwrap();
        let lua = include_bytes!("lua/script.zip");
        file.write_all(lua).unwrap();
    }

    // #[test]
    // fn t() {
    //     save_lua_scripts("a.zip");
    // }
}

mod local_monitor {
    use std::time::Duration;

    use tokio::time::sleep;

    use crate::{ColorId, Direction, Errors, Port, Side};

    use super::Vec2d;

    /// a monitor but stores the pixel localy,
    /// and can send only changed pixels
    ///
    /// x, y starts with 1
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LocalMonitor {
        data: Vec2d<AsIfPixel>,
        changed: Vec2d<bool>,
        wait_time: Duration,
        wait_count: usize,
    }

    /// as if a pixel, a basic display part of computer craft monitor
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AsIfPixel {
        text: char,
        pub background_color: ColorId,
        pub text_color: ColorId,
    }
    impl AsIfPixel {
        /// returns `None` if `text` is not within the ASCII range
        pub const fn new(
            text: char,
            background_color: ColorId,
            text_color: ColorId,
        ) -> Option<Self> {
            if !text.is_ascii() {
                None
            } else {
                Some(AsIfPixel {
                    text,
                    background_color,
                    text_color,
                })
            }
        }
        pub const fn colored_whitespace(color: ColorId) -> Self {
            AsIfPixel {
                text: ' ',
                background_color: color,
                text_color: color,
            }
        }
        pub fn text(&self) -> char {
            self.text
        }
    }
    // creating
    impl LocalMonitor {
        pub fn new(x: usize, y: usize, pixel: AsIfPixel) -> Self {
            Self {
                data: Vec2d::new_filled_copy(x, y, pixel),
                changed: Vec2d::new_filled_copy(x, y, true),
                wait_time: Duration::from_secs_f32(0.05),
                wait_count: 75,
            }
        }
        pub fn resize(&mut self, x: usize, y: usize, pixel: AsIfPixel) {
            self.data = Vec2d::new_filled_copy(x, y, pixel);
            self.changed = Vec2d::new_filled_copy(x, y, true);
        }
        pub fn size(&self) -> (usize, usize) {
            self.data.size()
        }
        pub fn x(&self) -> usize {
            self.data.x()
        }
        pub fn y(&self) -> usize {
            self.data.y()
        }
    }

    // useing
    impl LocalMonitor {
        /// x, y starts with 1
        pub fn get(&self, x: usize, y: usize) -> Option<AsIfPixel> {
            if x > self.x() || y > self.y() {
                None
            } else {
                let x = x - 1;
                let y = y - 1;
                Some(self.data[(x, y)])
            }
        }
        /// x, y starts with 1
        pub fn write(&mut self, x: usize, y: usize, pixel: AsIfPixel) {
            if x > self.x() || y > self.y() || x == 0 || y == 0 {
                return;
            }
            let x = x - 1;
            let y = y - 1;
            let p0 = self.data[(x, y)];
            if p0 != pixel {
                self.data[(x, y)] = pixel;
                self.changed[(x, y)] = true;
            }
        }

        pub fn clear_with(&mut self, color: ColorId) {
            for x in 1..=self.x() {
                for y in 1..=self.y() {
                    let pixel = AsIfPixel::colored_whitespace(color);
                    self.write(x, y, pixel);
                }
            }
        }

        /// write a [str], ignore non-ASCII chars
        pub fn write_str(
            &mut self,
            x: usize,
            y: usize,
            direction: Direction,
            text: &str,
            background_color: ColorId,
            text_color: ColorId,
        ) {
            let (dx, dy) = direction.to_dxdy();
            let (size_x, size_y) = self.size();
            let (size_x, size_y) = (size_x as isize, size_y as isize);

            let mut now_x = x as isize;
            let mut now_y = y as isize;
            #[allow(unused)]
            let (x, y) = ((), ());

            for c in text.chars() {
                let pixel = if let Some(p) = AsIfPixel::new(c, background_color, text_color) {
                    p
                } else {
                    continue;
                };
                self.write(now_x as usize, now_y as usize, pixel);
                now_x += dx;
                now_y += dy;
                if now_x <= 0 || now_x > size_x || now_y <= 0 || now_y > size_y {
                    return;
                }
            }
        }
    }

    impl LocalMonitor {
        pub async fn sync(&mut self, side: Side, port: &mut Port<'_>) -> Result<usize, Errors> {
            let mut count = 0;

            let mut pixels = Vec::new();

            for ((x, y), changed) in self.changed.iter() {
                if *changed {
                    let pixel = self.data[(x, y)];

                    pixels.push((x + 1, y + 1, pixel));

                    count += 1;
                }
            }

            port.monitor_write_multi(side, &pixels).await?;

            self.changed = Vec2d::new_filled_copy(self.x(), self.y(), false);
            Ok(count)
        }

        async fn _sync(&mut self, side: Side, port: &mut Port<'_>) -> Result<usize, Errors> {
            let mut sleep_counter = 0;
            let mut count = 0;
            for ((x, y), changed) in self.changed.iter() {
                if *changed {
                    if sleep_counter > self.wait_count {
                        sleep_counter = 0;
                        sleep(self.wait_time).await;
                    }

                    let pixel = self.data[(x, y)];
                    port.monitor_write(
                        side,
                        x + 1,
                        y + 1,
                        pixel.background_color,
                        pixel.text_color,
                        pixel.text,
                    )
                    .await?;
                    count += 1;
                    sleep_counter += 1;
                }
            }
            self.changed = Vec2d::new_filled_copy(self.x(), self.y(), false);
            Ok(count)
        }

        pub async fn sync_all(&mut self, side: Side, mut port: Port<'_>) -> Result<(), Errors> {
            let mut sleep_counter = 0;

            for ((x, y), pixel) in self.data.iter() {
                if sleep_counter > self.wait_count {
                    sleep_counter = 0;
                    sleep(self.wait_time).await;
                }
                port.monitor_write(
                    side,
                    x + 1,
                    y + 1,
                    pixel.background_color,
                    pixel.text_color,
                    pixel.text,
                )
                .await?;
                sleep_counter += 1;
            }
            self.changed = Vec2d::new_filled_copy(self.x(), self.y(), false);
            Ok(())
        }
    }
}

/// re-export some [futures] items for convince
///
/// [futures]: https://crates.io/crates/futures
pub mod futures {
    pub use futures::future::{join_all, FutureExt};
    pub use futures::TryFutureExt;
    #[test]
    fn t() {
        let a = async { 2 };
        let _ = a.then(|v| async move { Result::<_, ()>::Ok(v) });
    }
}

/// re-export some [tokio] items for convince
///
/// [tokio]: https://crates.io/crates/tokio
pub mod tokio {
    pub use tokio::main;
    pub use tokio::time::sleep;
}
