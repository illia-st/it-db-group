use std::rc::Rc;
use zmq::PollEvents;
use super::core::Socket;

pub struct Poller {
    sockets: Vec<Rc<dyn Socket>>,
}

impl Default for Poller {
    fn default() -> Self {
        Poller::new()
    }
}

impl Poller {
    pub fn new() -> Self {
        Poller { sockets: Vec::new() }
    }
    pub fn add(&mut self, socket: Rc<dyn Socket>) -> &mut Self {
        self.sockets.push(socket);
        self
    }
    pub fn poll(&mut self, poll_count: i32) -> Vec<Option<Vec<u8>>> {
        let mut items = Vec::new();
        let mut counter = 0;
        for socket in &self.sockets {
            let poll_item = socket.get_socket().as_poll_item(PollEvents::POLLIN);
            items.push(poll_item);
        }
        let mut ans = Vec::default();
        while counter != poll_count {
            zmq::poll(&mut items, -1).expect("polling error");
            for (index, item) in items.iter().enumerate() {
                if item.is_readable() {
                    counter += 1;
                    let socket = &self.sockets[index];
                    ans.push(socket.handle(socket.get_receiver(), socket.get_sender()));
                }
            }
        }
        ans
    }
}
