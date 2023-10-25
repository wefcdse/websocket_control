use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use websocket_control::utils::futures::join_all;
use websocket_control::{ColorId, Event, Ports, Side, ToErrorsResult};

#[websocket_control::utils::tokio::main]
async fn main() {
    #[allow(unused)]
    if false {
        tokio::spawn(async {
            let time = Instant::now();
            let a = tokio::time::sleep(Duration::from_secs(3));
            let vec = (0..10000)
                .map(|_| tokio::time::sleep(Duration::from_secs(3)))
                .collect::<Vec<_>>();

            let o = futures::future::join_all(vec.into_iter()).await;
            dbg!(o.len());
            dbg!(time.elapsed());
        })
        .await
        .unwrap();
        return;
    }
    env_logger::Builder::new()
        // .filter_level(log::LevelFilter::Trace)
        .filter_module(
            "websocket_control::get_router_with_tick_func",
            log::LevelFilter::Debug,
        )
        .filter_module("websocket_control::get_router", log::LevelFilter::Trace)
        .filter_module("websocket_control", log::LevelFilter::Debug)
        .init();

    websocket_control::serve_tick_func(&([127, 0, 0, 1], 14111).into(), tick3, Instant::now())
        .await;
}

async fn tick(
    state: &mut f32,
    ports: Ports<'_>,
    dt: Duration,
) -> Result<(), websocket_control::Errors> {
    // tokio::time::sleep(Duration::from_secs_f32((1.0 - dt.as_secs_f32()).max(0.))).await;

    // println!("{:?}", dt);
    let mut ports = ports.all_ports();

    let a = join_all(ports.iter_mut().map(|(_id, p)| p.get_redstone(Side::Left)))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    join_all(
        ports
            .iter_mut()
            .enumerate()
            .map(|(id, (_id, p))| p.set_redstone(Side::Right, (a[id] + 1).min(15))),
    )
    .await
    .into_iter()
    .collect::<Result<_, _>>()?;

    *state += dt.as_secs_f32() * 4.;
    while *state > 31. {
        *state -= 31.;
    }

    let rs_level = if *state < 16. {
        *state as i32
    } else {
        (31. - *state) as i32
    };

    let mut futs = Vec::new();
    for (_id, ws) in &mut ports {
        let fut = ws.set_redstone(Side::Front, rs_level);
        // dbg!(std::mem::size_of_val(&fut));
        futs.push(fut);
    }
    let rtn = futures::future::join_all(futs).await;
    for r in rtn {
        r?;
    }

    let mut futs = Vec::new();
    for (_id, ws) in &mut ports {
        let fut = ws.gps_locate();
        // dbg!(std::mem::size_of_val(&fut));
        futs.push(fut);
    }
    let rtn = futures::future::join_all(futs).await;
    for r in rtn {
        r?;
    }

    Ok(())
}

async fn tick1(
    state: &mut (),
    ports: Ports<'_>,
    dt: Duration,
) -> Result<(), websocket_control::Errors> {
    let mut ports = ports.all_ports();
    // dbg!(ports.len());
    for (id, p) in &mut ports {
        // if let Ok(msg) = p.get_peripheral(Side::Top).await {
        //     dbg!(msg);
        // }
        // dbg!(
        //     p.monitor_write(Side::Top, 5, 5, ColorId::C04, ColorId::C05, "ads")
        //         .await
        // );

        dbg!(p.monitor_get_size(Side::Top).await);
    }
    // dbg!();
    Ok(())
}

async fn tick2(
    state: &mut (u16, u16, u16, u16, Instant),
    mut ports: Ports<'_>,
    dt: Duration,
) -> Result<(), websocket_control::Errors> {
    let (mut sizex, mut sizey, mut x, mut y, mut t) = *state;
    let mut p1 = ports.get_port("p1").to_errors_result()?;
    if let Some(evt) = p1.pull_event().await? {
        match evt {
            Event::MonitorTouch {
                side: Side::Top,
                x: x1,
                y: y1,
            } => {
                if (x, y) == (x1, y1) {
                    (sizex, sizey) = p1.monitor_get_size(Side::Top).await?.to_errors_result()?;
                    p1.monitor_write(Side::Top, x, y, ColorId::C16, ColorId::C16, ' ')
                        .await?;
                    x = rand::random::<u16>() % sizex + 1;
                    y = rand::random::<u16>() % sizey + 1;
                    p1.monitor_write(Side::Top, x, y, ColorId::C05, ColorId::C01, ' ')
                        .await?;
                    t = Instant::now();
                }
            }
            _ => {}
        }
    }

    if t.elapsed().as_secs_f32() > 0.5 {
        p1.monitor_write(Side::Top, x, y, ColorId::C05, ColorId::C01, ' ')
            .await?;
        tokio::time::sleep(Duration::from_secs(0)).await;
        t = Instant::now();
    }

    *state = (sizex, sizey, x, y, t);
    Ok(())
}

async fn tick3(
    state: &mut Instant,
    mut ports: Ports<'_>,
    dt: Duration,
) -> Result<(), websocket_control::Errors> {
    let mut t = *state;
    let mut p1 = ports.get_port("p1").to_errors_result()?;

    let mut counter = 0;

    if t.elapsed().as_secs_f32() > 2.0 {
        let (xmax, ymax) = p1.monitor_get_size(Side::Top).await?.to_errors_result()?;
        for x in 1..=xmax {
            for y in 1..=ymax {
                if counter > 100 {
                    tokio::time::sleep(Duration::from_secs_f32(0.04)).await;
                    counter = 0;
                }
                counter += 1;
                p1.monitor_write(Side::Top, x, y, ColorId::C04, ColorId::C12, ' '.into())
                    .await?;
            }
        }
        t = Instant::now();
    }

    *state = t;
    Ok(())
}
