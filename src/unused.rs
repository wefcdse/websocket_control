#![allow(unused)]
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::time::sleep;
use websocket_control::{ColorId, Ports, Side, ToErrorsResult};

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

async fn tick3(
    state: &mut Instant,
    mut ports: Ports<'_>,
    dt: Duration,
) -> Result<(), websocket_control::Errors> {
    dbg!(ports.len());
    sleep(Duration::from_secs_f32(1.0)).await;
    let mut t = *state;
    let mut p1 = ports.get_port("p1").to_errors_result()?;

    let mut counter = 0;

    if t.elapsed().as_secs_f32() > 2.0 {
        let (xmax, ymax) = p1.monitor_get_size(Side::Top).await?.to_errors_result()?;
        for x in 1..=xmax {
            for y in 1..=ymax {
                if counter > 100 {
                    sleep(Duration::from_secs_f32(0.04)).await;
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
