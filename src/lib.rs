// use runtime_raw::*;

// use futures::future::BoxFuture;
// use futures::task::SpawnError;

// use std::io;
// use std::net::SocketAddr;
// use std::pin::Pin;
// use std::time::{Duration, Instant};

pub struct Golang {}

// impl Runtime for Golang {
// fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {}

// fn connect_tcp_stream(
//     &self,
//     addr: &SocketAddr,
// ) -> BoxFuture<'static, io::Result<Pin<Box<dyn TcpStream>>>> {
// }

// fn bind_tcp_listener(&self, addr: &SocketAddr) -> io::Result<Pin<Box<dyn TcpListener>>> {}

// fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Pin<Box<dyn UdpSocket>>> {}

// fn new_delay(&self, dur: Duration) -> Pin<Box<dyn Delay>> {}

// fn new_delay_at(&self, at: Instant) -> Pin<Box<dyn Delay>> {}

// fn new_interval(&self, dur: Duration) -> Pin<Box<dyn Interval>> {}
// }
