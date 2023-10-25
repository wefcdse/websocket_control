use std::{
    error::Error,
    future::Future,
    net::SocketAddr,
    thread,
    time::{Duration, Instant},
};

use axum::Router;
use tokio::{runtime::Runtime, task::AbortHandle};

pub trait UseAsTickFunc<'a, State: Send>: Send {
    type Err: Error;
    type Fut: 'a + Future<Output = Result<(), Self::Err>> + Send;
    fn call(&mut self, state: &'a mut State, ports: Ports<'a>, dt: Duration) -> Self::Fut;
}

impl<'a, State, Err, Fut, F> UseAsTickFunc<'a, State> for F
where
    State: 'static + Send,
    F: FnMut(&'a mut State, Ports<'a>, Duration) -> Fut + Send,
    Fut: 'a + Future<Output = Result<(), Err>> + Send,
    Err: Error,
{
    type Err = Err;

    type Fut = Fut;

    fn call(&mut self, state: &'a mut State, ports: Ports<'a>, dt: Duration) -> Self::Fut {
        self(state, ports, dt)
    }
}

use crate::{get_router, Ports, SocketCollectionStateHandle};
/// tick func is:
/// ```ignore
/// async fn(&mut {Some State}, Ports<'_>, Duration) -> Result<(),{Some type impl Error}>
/// ```
pub fn get_router_with_tick_func<F, State>(
    mut tick: F,
    state: State,
) -> (Router, AbortHandle, SocketCollectionStateHandle)
where
    State: 'static + Send,
    F: 'static + for<'a> UseAsTickFunc<'a, State>,
{
    get_router(|ws| async move {
        let mut ws = ws;
        let mut time = Instant::now();
        let mut state = state;
        loop {
            ws.collect_connections();
            ws.clean();

            let dt = time.elapsed();
            time = Instant::now();
            log::trace!("tick time:{:?}", dt);
            match tick.call(&mut state, ws.ports(), dt).await {
                Ok(_) => {}
                Err(e) => {
                    // log::error!("Error: {}", e);
                }
            }
        }
    })
}

/// tick func is like this:
/// ```ignore
/// async fn(&mut {Some State}, Ports<'_>, Duration) -> Result<(),{Some type impl Error}>
/// ```
pub fn serve_tick_func<F, State>(addr: &SocketAddr, tick: F, state: State) -> ()
where
    State: 'static + Send + Clone,
    F: 'static + for<'a> UseAsTickFunc<'a, State> + Clone,
{
    let addr = *addr;
    let loop1 = async move {
        let (app, mut main, mut sc) = get_router_with_tick_func(tick.clone(), state.clone());
        let fut = axum::Server::bind(&addr).serve(app.into_make_service());
        let mut axum = tokio::spawn(fut).abort_handle();

        loop {
            tokio::time::sleep(Duration::from_secs_f32(0.5)).await;
            let inited = sc.ws_added.load(std::sync::atomic::Ordering::Relaxed);
            if inited {
                if sc.ws_count.load(std::sync::atomic::Ordering::Relaxed) <= 0 {
                    log::info!("restart");
                    axum.abort();
                    main.abort();
                    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;
                    let (app, main1, sc1) = get_router_with_tick_func(tick.clone(), state.clone());
                    let fut = axum::Server::bind(&addr).serve(app.into_make_service());
                    let axum1 = tokio::spawn(fut).abort_handle();
                    main = main1;
                    axum = axum1;
                    sc = sc1;
                }
            }
        }
    };
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(loop1);
}

fn auto_restart<F, State>(
    addr: &SocketAddr,
    tick: F,
    state: State,
    rt: Runtime,
    mut main: AbortHandle,
    mut axum: AbortHandle,
    mut sc: SocketCollectionStateHandle,
) where
    State: 'static + Send + Clone,
    F: 'static + for<'a> UseAsTickFunc<'a, State> + Clone,
{
    loop {
        thread::sleep(Duration::from_secs_f32(0.5));
        let inited = sc.ws_added.load(std::sync::atomic::Ordering::Relaxed);
        if inited {
            if sc.ws_count.load(std::sync::atomic::Ordering::Relaxed) <= 0 {
                log::info!("restart");
                axum.abort();
                main.abort();
                thread::sleep(Duration::from_secs_f32(0.5));
                let tick = tick.clone();
                let state = state.clone();
                let f1 = async move {
                    let (app, main1, sc1) = get_router_with_tick_func(tick, state);
                    let fut = axum::Server::bind(&addr).serve(app.into_make_service());
                    let axum1 = tokio::spawn(fut).abort_handle();
                    (main1, axum1, sc1)
                };
                let (main1, axum1, sc1) = rt.block_on(f1);
                main = main1;
                axum = axum1;
                sc = sc1;
            }
        }
    }
}
