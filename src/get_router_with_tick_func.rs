
use std::{
    error::Error,
    future::Future,
    time::{Duration, Instant},
};

use axum::Router;

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

use crate::{get_router, Ports};
/// tick func is:
/// ```rust
/// async fn(&mut {Some State},Ports<'_>, Duration) -> Result<(),{Some type impl Error}>
/// ```
pub fn get_router_with_tick_func<F, State>(mut tick: F, state: State) -> Router
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

            match tick.call(&mut state, ws.ports(), dt).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("Error: {}", e);
                }
            }
        }
    })
}
