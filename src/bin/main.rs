use std::io;
use std::io::Write;
use chess_engine::engine;

fn main() -> io::Result<()> {

    let e = engine::Engine;

    e.consume_input(io::stdin(), io::stdout());

    // let stdin = io::stdin();
    // let mut stdout = io::stdout();
    //
    // let mut moves_sent = 0;
    //
    // for line in stdin.lines() {
    //     let line = match line {
    //         Ok(line) => line,
    //         Err(_) => break,
    //     };
    //
    //     let mut tokens = line.split_whitespace();
    //     let front_token = tokens.next();
    //     match front_token {
    //         Some(front_token) => {
    //             match front_token {
    //                 "uci" => {
    //                     stdout.write_all("id name basic_chess_engine \n".as_ref()).expect("requires stdout");
    //                     stdout.write_all("id author andrew frauens \n".as_ref()).expect("requires stdout");
    //                     stdout.write_all("uciok\n".as_ref()).expect("requires stdout");
    //                 }
    //                 "debug" => {},
    //                 "isready" => {
    //                     stdout.write_all("readyok\n".as_ref()).expect("requires stdout");
    //                 },
    //                 "setoption" => {},
    //                 "register" => {},
    //                 "ucinewgame" => {},
    //                 "position" => {},
    //                 "go" => {
    //                     if moves_sent %2 == 0 {
    //                         stdout.write_all("bestmove e2e4\n".as_ref()).expect("requires stdout");
    //                     } else {
    //                         stdout.write_all("bestmove e1e2\n".as_ref()).expect("requires stdout");
    //                     }
    //                     moves_sent += 1;
    //                 },
    //                 "stop" => {
    //                     if moves_sent %2 == 0 {
    //                         stdout.write_all("bestmove e2e4\n".as_ref()).expect("requires stdout");
    //                     } else {
    //                         stdout.write_all("bestmove e1e2\n".as_ref()).expect("requires stdout");
    //                     }
    //                     moves_sent += 1;
    //                 },
    //                 "ponderhit" => {},
    //                 "quit" => {return Ok(())},
    //                 _ => {}
    //             }
    //         }
    //         None => {}
    //     }
    // }

    Ok(())
}
