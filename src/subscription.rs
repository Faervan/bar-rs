use std::{fmt::Debug, sync::Arc, time::Duration};

use iced::{
    event::{listen_with, wayland, PlatformSpecific},
    futures::{future::BoxFuture, SinkExt},
    stream,
};
use log::warn;
use tokio::{
    sync::{mpsc, oneshot},
    time::sleep,
};

use crate::{
    message::{get_config, UpdateFn},
    Message, State,
};

impl State {
    pub fn subscribe(&self) -> iced::Subscription<Message> {
        let module_subs = iced::Subscription::run(|| {
            stream::channel(1, |mut sender| async move {
                let (sx, rx) = oneshot::channel();
                sender.send(Message::FetchSubscriptions(sx)).await.unwrap();
                let subs = rx.await.unwrap();
                let (sx, mut rx) = mpsc::channel(1);

                let config = get_config(&mut sender).await;

                for sub in subs {
                    sub.run(&sx).await;
                }

                let mut updates = vec![];
                loop {
                    tokio::select! {
                        r = rx.recv() => match r {
                            Some(SubscriptionUpdate::Buffered(update)) => {
                                updates.push(update);
                                continue;
                            }
                            Some(SubscriptionUpdate::Immediate(update)) => updates.push(update),
                            None => {
                                warn!("All Subscription senders dropped, canceling Subscription");
                                return;
                            }
                        },
                        _ = sleep(Duration::from_secs_f64(config.reload_interval)) => {}
                    }
                    sender
                        .send(Message::Update(std::mem::take(&mut updates)))
                        .await
                        .unwrap();
                }
            })
        });

        iced::Subscription::batch([
            listen_with(|event, _, _| {
                if let iced::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::Output(event, wl_output),
                )) = event
                {
                    Some(Message::OutputEvent { event, wl_output })
                } else {
                    None
                }
            }),
            module_subs,
        ])
    }
}

pub enum SubscriptionUpdate {
    Buffered(UpdateFn),
    Immediate(UpdateFn),
}

impl SubscriptionUpdate {
    pub fn buffered<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self::Buffered(UpdateFn(Arc::new(Box::new(f))))
    }

    pub fn immediate<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self::Immediate(UpdateFn(Arc::new(Box::new(f))))
    }
}

pub struct Subscription(
    Option<
        Box<
            dyn FnOnce(mpsc::Sender<SubscriptionUpdate>) -> BoxFuture<'static, ()>
                + Send
                + Sync
                + 'static,
        >,
    >,
);

impl Debug for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Subscription(Option<Box<dyn FnOnce(mpsc::Sender<SubscriptionUpdate>)
            -> BoxFuture<'static, ()> + 'static>>)"
        )
    }
}

impl Subscription {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(mpsc::Sender<SubscriptionUpdate>) -> BoxFuture<'static, ()>
            + Send
            + Sync
            + 'static,
    {
        Self(Some(Box::new(f)))
    }

    pub fn none() -> Self {
        Self(None)
    }

    async fn run(self, sx: &mpsc::Sender<SubscriptionUpdate>) {
        if let Some(f) = self.0 {
            let sx = sx.clone();
            tokio::spawn(f(sx));
        }
    }
}
