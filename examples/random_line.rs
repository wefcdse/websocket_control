/// run with `cargo run --example random_line`
///
/// in minecraft:
/// place a monitor on the top of a computer
/// run `ws_control p1 14111`
use computercraft_websocket_control::{
    serve_tick_func,
    utils::{AsIfPixel, LocalMonitor},
    ColorId, Errors, Ports, Side, ToErrorsResult,
};
use std::time::Duration;
fn main() {
    env_logger::Builder::new()
        .filter_module(
            "computercraft_websocket_control::get_router_with_tick_func",
            log::LevelFilter::Debug,
        )
        .filter_module(
            "computercraft_websocket_control::get_router",
            log::LevelFilter::Trace,
        )
        .filter_module("computercraft_websocket_control", log::LevelFilter::Debug)
        .init();

    serve_tick_func(
        &([127, 0, 0, 1], 14111).into(),
        tick,
        LocalMonitor::new(0, 0, AsIfPixel::colored_whitespace(ColorId::Orange)),
    );
}

async fn tick(state: &mut LocalMonitor, mut ports: Ports<'_>, _dt: Duration) -> Result<(), Errors> {
    let mut p1 = ports.get_port("p1").to_errors_result()?;
    state.sync(Side::Top, &mut p1).await?;

    let (size_x, size_y) = p1.monitor_get_size(Side::Top).await?.to_errors_result()?;
    if state.size() != (size_x, size_y) {
        let pixel = AsIfPixel::new(' ', ColorId::Lime, ColorId::Orange).unwrap();
        state.resize(size_x, size_y, pixel);
    }

    let y = (size_y + 1) / 2;
    for x in 1..size_x + 1 {
        let c1 = ColorId::from_number_overflow(rand::random());
        let c2 = ColorId::from_number_overflow(rand::random());
        let text = char::from_u32(rand::random::<u32>() % 26 + 65).unwrap();

        state.write(x, y, AsIfPixel::new(text, c1, c2).unwrap());
    }

    Ok(())
}
