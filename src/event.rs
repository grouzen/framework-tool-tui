use std::time::Duration;

use color_eyre::eyre::Report;
use crossterm::event::EventStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

pub enum Event {
    Tick,
    Input(crossterm::event::Event),
}

pub struct EventLoop {
    tx: mpsc::UnboundedSender<Event>,
    rx: mpsc::UnboundedReceiver<Event>,
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl EventLoop {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self { tx, rx }
    }

    pub fn run(&self, tick_interval: Duration) {
        let tx = self.tx.clone();

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
                }
            }
        });
    }

    pub async fn next(&mut self) -> color_eyre::Result<Event> {
        self.rx
            .recv()
            .await
            .ok_or(Report::msg("Event loop async channel error"))
    }
}
