use std::time::Duration;

use color_eyre::eyre::Report;
use crossterm::event::EventStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, watch};

pub enum Event {
    Tick,
    Input(crossterm::event::Event),
}

pub struct EventLoop {
    tx: mpsc::UnboundedSender<Event>,
    rx: mpsc::UnboundedReceiver<Event>,
    interval_tx: watch::Sender<Duration>,
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl EventLoop {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let (interval_tx, _interval_rx) = watch::channel(Duration::from_millis(1000));

        Self {
            tx,
            rx,
            interval_tx,
        }
    }

    pub fn run(&self, tick_interval: Duration) {
        let tx = self.tx.clone();
        let mut interval_rx = self.interval_tx.subscribe();

        tokio::spawn(async move {
            let mut event_stream = EventStream::new();
            let mut tick = tokio::time::interval(tick_interval);

            loop {
                let tick_event = tick.tick();
                let input_event = event_stream.next().fuse();

                tokio::select! {
                    _ = tick_event => {
                        let _ = tx.send(Event::Tick);
                    }
                    Some(Ok(event)) = input_event => {
                        let _ = tx.send(Event::Input(event));
                    }
                    Ok(()) = interval_rx.changed() => {
                        let new_interval = *interval_rx.borrow_and_update();
                        tick = tokio::time::interval(new_interval);
                    }
                }
            }
        });
    }

    pub fn set_tick_interval(&self, tick_interval: Duration) {
        let _ = self.interval_tx.send(tick_interval);
    }

    pub async fn next(&mut self) -> color_eyre::Result<Event> {
        self.rx
            .recv()
            .await
            .ok_or(Report::msg("Event loop async channel error"))
    }
}
