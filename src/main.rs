mod unused;

use std::time::{Duration, Instant};
use websocket_control::{ColorId, Event, Ports, Side, ToErrorsResult};
fn main() {
    env_logger::Builder::new()
        .filter_module(
            "websocket_control::get_router_with_tick_func",
            log::LevelFilter::Debug,
        )
        .filter_module("websocket_control::get_router", log::LevelFilter::Trace)
        .filter_module("websocket_control", log::LevelFilter::Debug)
        .init();

    websocket_control::serve_tick_func(
        &([127, 0, 0, 1], 14111).into(),
        tick,
        (0, 0, 1, 1, Instant::now()),
    );
}

async fn tick(
    state: &mut (u16, u16, u16, u16, Instant),
    mut ports: Ports<'_>,
    _dt: Duration,
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
