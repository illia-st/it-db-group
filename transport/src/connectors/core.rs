pub trait Handler {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender);
}

pub trait Receiver {
    fn recv(&self) -> Vec<u8>;
}

pub trait Sender {
    fn send(&self, data: &[u8]);
}


pub trait Socket {

    fn get_socket(&self) -> &zmq::Socket;

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender);

    fn get_receiver(&self) -> &dyn Receiver;

    fn get_sender(&self) -> &dyn Sender;
}
