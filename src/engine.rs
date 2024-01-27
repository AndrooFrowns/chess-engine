use std::{io, thread};
use std::io::BufRead;
use std::sync::{mpsc, Mutex};
use std::sync::mpsc::{Receiver, TryRecvError};
use vampirc_uci::{parse_one, UciMessage};
use vampirc_uci::UciMessage::Uci;
use crate::board::Board;

pub struct Engine<T, U> {
    board: Board,

    // input_stream: Mutex<T>,
    // output_stream: Mutex<U>,
}


impl Engine<T, U> {
    pub fn new() -> Self {
        Engine { board: Board {} }
    }

    fn run(mut self) {
        let stdin_channel = spawn_stdin_channel();

        loop {
            match stdin_channel.try_recv() {
                Ok(key) => println!("Received: {}", key),
                Err(TryRecvError::Empty) => println!("Channel empty"),
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }
            // sleep(1000);
        }
    }


    fn spawn_stdin_channel(mut self) -> Receiver<UciMessage> {
        let (tx, rx) = mpsc::channel::<UciMessage>();

        thread::spawn(move || loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

            for line in io::stdin().lock().lines() {
                let msg: UciMessage = parse_one(&line.unwrap());

                if msg == UciMessage::Quit {
                    tx.send(msg).unwrap();
                    return;
                }

                tx.send(msg).unwrap();
            }
        });

        rx
    }


    fn process_input_message(msg: UciMessage) {
        match msg {
            UciMessage::Uci => {}
            UciMessage::Debug(_) => {}
            UciMessage::IsReady => {}
            UciMessage::Register { .. } => {}
            UciMessage::Position { .. } => {}
            UciMessage::SetOption { .. } => {}
            UciMessage::UciNewGame => {}
            UciMessage::Stop => {}
            UciMessage::PonderHit => {}
            UciMessage::Quit => {}
            UciMessage::Go { .. } => {}
            UciMessage::Id { .. } => {}
            UciMessage::UciOk => {}
            UciMessage::ReadyOk => {}
            UciMessage::BestMove { .. } => {}
            UciMessage::CopyProtection(_) => {}
            UciMessage::Registration(_) => {}
            UciMessage::Option(_) => {}
            UciMessage::Info(_) => {}
            UciMessage::Unknown(_, _) => {}
        };
    }

    fn consume_input() {
        for line in io::stdin().lock().lines() {
            let msg: UciMessage = parse_one(&line.unwrap());
            println!("Received message: {}", msg);
            // match msg {}
        }
    }
}

