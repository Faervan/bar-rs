use hyprland::{data::{Client, Monitor, Workspaces}, event_listener::AsyncEventListener, keyword::Keyword, shared::{HyprData, HyprDataActive, HyprDataActiveOptional, HyprDataVec}};
use iced::{futures::{channel::mpsc::Sender, SinkExt, Stream}, stream};

use crate::{Message, BAR_HEIGHT};

#[derive(Debug, Default, Clone)]
pub struct OpenWorkspaces {
    pub active: usize,
    pub open: Vec<String>,
}

impl From<(Workspaces, usize)> for OpenWorkspaces {
    fn from(value: (Workspaces, usize)) -> Self {
        let mut workspaces = OpenWorkspaces {
            active: value.1,
            open: vec![],
        };
        let mut value = value.0.to_vec();
        value.sort_by(|a, b| a.id.cmp(&b.id));
        value.iter()
            .for_each(
                |ws| workspaces.open.push(ws.name.clone())
            );
        workspaces
    }
}

pub fn hyprland_events() -> impl Stream<Item = Message> {
    stream::channel(1, |mut sender| async move {

        update_workspaces(&mut sender, None).await;
        if let Ok(Some(window)) = Client::get_active() {
            sender.send(Message::Window(Some(window.title)))
                .await
                .unwrap_or_else(|err| {
                    eprintln!("Trying to send workspaces failed with err: {err}");
                });
        }

        let mut listener = AsyncEventListener::new();

        let sender1 = sender.clone();
        listener.add_active_window_changed_handler(move |data| {
            let mut sender = sender1.clone();
            Box::pin(async move {
                sender
                    .send(Message::Window(data.map(|name| match name.title.len() > 25 {
                        true => format!(
                            "{}...",
                            &name.title
                                .chars()
                                .take(22)
                                .collect::<String>()
                        ),
                        false => name.title
                    })))
                    .await
                    .unwrap();
            })
        });

        listener.add_workspace_changed_handler(move |data| {
            let mut sender = sender.clone();
            Box::pin(async move {
                update_workspaces(&mut sender, Some(data.id)).await;
            })
        });

        listener.add_config_reloaded_handler(||
            Box::pin(async {
                reserve_bar_space()
            })
        );

        listener.start_listener_async().await
            .expect("Failed to listen for hyprland events");
    })
}

async fn update_workspaces(sender: &mut Sender<Message>, active: Option<i32>) {
    if let Ok(workspaces) = Workspaces::get() {
        sender.send(Message::Workspaces(
            OpenWorkspaces::from((
                workspaces,
                active.unwrap_or(
                    Monitor::get_active()
                        .map(|monitor| monitor.active_workspace.id)
                        .unwrap_or(0)
                ) as usize - 1,
            )
        )))
        .await
        .unwrap_or_else(|err| {
            eprintln!("Trying to send workspaces failed with err: {err}");
        });
    }
}

pub fn reserve_bar_space() {
    Keyword::set("monitor", format!("eDP-1, addreserved, {BAR_HEIGHT}, 0, 0, 0"))
        .expect("Failed to set reserved space using hyprctl");
}
