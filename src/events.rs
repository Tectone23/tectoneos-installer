use std::{any::TypeId, time::Duration};

use iced_native::{
    futures::{
        channel::mpsc::{self, Sender},
        SinkExt,
    },
    subscription, Subscription,
};

use crate::routes::AppPages;

pub enum Message {
    CheckPrereq,
}

enum State {
    Init,
    Ready(mpsc::Receiver<Message>),
}

#[derive(Clone, Debug)]
pub enum Event {
    Connected(Connection),
    Navigate(AppPages),
}

#[derive(Clone, Debug)]
pub struct Connection(Sender<Message>);
impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0.try_send(message).expect("SENDING MESSAGE FAILED");
    }
}

pub fn task_worker() -> Subscription<Event> {
    struct Task;

    return subscription::channel(TypeId::of::<Task>(), 100, |mut output| async move {
        let mut state = State::Init;

        loop {
            match &mut state {
                State::Init => {
                    let (sender, receiver) = mpsc::channel(100);
                    let _ = output.send(Event::Connected(Connection(sender))).await;
                    state = State::Ready(receiver);
                }
                State::Ready(receiver) => {
                    use iced_native::futures::StreamExt;
                    let input = receiver.select_next_some().await;

                    match input {
                        Message::CheckPrereq => {
                            tokio::time::sleep(Duration::from_secs(3)).await;
                            // if this fails, then we have no way of letting the user know
                            // on the front end. And logging the error wont change much
                            // for now, I will just drop the error
                            let _ = output.send(Event::Navigate(AppPages::Eula)).await;
                        }
                    }
                }
            };
        }
    });
}
