use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32; // buffer size

fn sleep() {
    thread::sleep(std::time::Duration::from_millis(3000));
}

fn main() {
    let server_socket = TcpListener::bind(LOCAL).expect("Listener failed to bind...");
    // set server to non-block-mode to let the server check constantly for messages
    server_socket.set_nonblocking(true).expect("failed to initialize non-blocking...");

    // let multiple clients use the channel
    let mut clients = vec![];
    // create a channel and assign it to a string type
    // we are telling the channel that we send a bunch of strings through the channel
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut connection_socket, addr)) = server_socket.accept() {
            println!("Client {} connected, socket {:?}", addr, connection_socket.local_addr());
            // 2:30 min
            let tx = tx.clone();
            println!("Tx: {:?}", tx);
            // clone socket to push it into our thread
            clients.push(connection_socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match connection_socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{}: {:?}", addr, msg);
                        // tx.send(String::from("bbbbb")).expect("failed to send msg to rx");  
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }
                sleep();
            });
        }
        if let Ok(msg) = rx.try_recv() {
            println!("Receiver msg: {}", msg);
            println!("Tx msg: {:?}", tx);
            clients = clients.into_iter().filter_map(|mut socket| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                socket.write_all(&buff).map(|_| socket).ok()
            }).collect::<Vec<_>>();
        }
        sleep();

    }

}
