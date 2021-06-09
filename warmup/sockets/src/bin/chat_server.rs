use sockets::{recv_message, send_message};
use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type SocketList = Vec<TcpStream>;

fn acceptor(sockets: Arc<Mutex<SocketList>>, tx: Sender<String>) -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:12345")?;

    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                let cloned_sock = socket.try_clone().unwrap();
                let mut sockets_unlock = sockets.lock().unwrap();
                sockets_unlock.push(cloned_sock);
                let tx1 = Sender::clone(&tx);
                thread::spawn(move || {
                    receiver(socket, tx1);
                });
            }

            Err(e) => {
                println!("couldn't get client: {:?}", e);
            }
        }
    }
}

fn receiver(mut socket: TcpStream, tx: Sender<String>) -> io::Result<()> {
    loop {
        let msg = recv_message(socket.try_clone().unwrap())?;
        tx.send(msg).unwrap();
    }
}

fn sender(sockets: Arc<Mutex<SocketList>>, rx: Receiver<String>) -> io::Result<()> {
    loop {
        let msg = rx.recv().unwrap();
        let sockets = sockets.lock().unwrap();
        for sock in &*sockets {
            send_message(sock, msg.clone())?;
        }
    }
}

fn main() -> std::io::Result<()> {
    let (tx, rx) = channel();
    let sockets = Arc::new(Mutex::new(vec![]));
    let sockets1 = Arc::clone(&sockets);
    let sockets2 = Arc::clone(&sockets);
    let acceptor_handle = thread::spawn(move || {
        acceptor(sockets1, tx);
    });
    let sender_handle = thread::spawn(move || {
        sender(sockets2, rx);
    });
    acceptor_handle.join().unwrap();
    sender_handle.join().unwrap();
    Ok(())
}