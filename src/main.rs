use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use futures::future::join_all;
use websocket_control::{Ports, Side};

#[tokio::main]
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
            log::LevelFilter::Trace,
        )
        .filter_module("websocket_control", log::LevelFilter::Trace)
        .init();

    let app = websocket_control::get_router_with_tick_func(tick, 0.);

    let addr = SocketAddr::from(([127, 0, 0, 1], 14111));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn tick(
    state: &mut f32,
    ports: Ports<'_>,
    dt: Duration,
) -> Result<(), websocket_control::Errors> {
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

    tokio::time::sleep(Duration::from_secs_f32(
        (1.0 / 20. - dt.as_secs_f32()).max(0.),
    ))
    .await;
    Ok(())
}
