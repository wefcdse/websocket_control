pub use vec2d::Vec2d;
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
    /// use websocket_control::utils::Vec2d;
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

    impl<T: Display> Display for Vec2d<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in 0..self.y {
                for x in 0..self.x {
                    write!(f, "{} ", self[(x, y)])?;
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
        vec![4][2];
    }
}

mod local_monitor {
    use std::time::Duration;

    use tokio::time::sleep;

    use crate::{ColorId, Errors, Port, Side};

    use super::Vec2d;

    pub struct LocalMonitor {
        data: Vec2d<AsIfPixel>,
        changed: Vec2d<bool>,
        wait_time: Duration,
        wait_count: usize,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AsIfPixel {
        txt: char,
        background_color: ColorId,
        text_color: ColorId,
    }

    impl LocalMonitor {
        pub fn new(x: usize, y: usize, pix: AsIfPixel) -> Self {
            Self {
                data: Vec2d::new_filled_copy(x, y, pix),
                changed: Vec2d::new_filled_copy(x, y, true),
                wait_time: Duration::from_secs_f32(0.05),
                wait_count: 100,
            }
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

    impl LocalMonitor {
        pub async fn sync(&mut self, side: Side, mut port: Port<'_>) -> Result<usize, Errors> {
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
                        x as u16,
                        y as u16,
                        pixel.background_color,
                        pixel.text_color,
                        pixel.txt,
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
                    x as u16,
                    y as u16,
                    pixel.background_color,
                    pixel.text_color,
                    pixel.txt,
                )
                .await?;
                sleep_counter += 1;
            }
            self.changed = Vec2d::new_filled_copy(self.x(), self.y(), false);
            Ok(())
        }
    }
}

/// reexport some [futures] items for convince
pub mod futures {
    pub use futures::future::{join_all, FutureExt};
    pub use futures::TryFutureExt;
    #[test]
    fn t() {
        let a = async { 2 };
        let _ = a.then(|v| async move { Result::<_, ()>::Ok(v) });
    }
}

/// reexport some [tokio][extern crate tokio] items for convince
pub mod tokio {
    pub use tokio::main;
    pub use tokio::time::sleep;
}
