use std::{env, process};
use compilador_de_calculos::{run, Config};

// fn main() {
//     // let args: Vec<String> = env::args().collect();
//     //
//     // let config = Config::new(&args).unwrap_or_else(|err| {
//     //     println!("Problem parsing arguments: {}", err);
//     //     process::exit(1);
//     // });
//     //
//     //  run(config);
//
// }


use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Colorize}, Result, execute, queue,
};
use crossterm::event::{read, Event};
use std::thread::current;
use crossterm::event::KeyCode::{Char, Backspace};
use compilador_de_calculos::lexer::lexic_analize;
use regex::Error;
use compilador_de_calculos::token::Token::{*};

fn update_string(s: &mut String) {
    match read().unwrap() {
        Event::Key(event) => {
            println!("{:?}", event);
            match event.code {
                Char(letter) => s.push(letter),
                Backspace => { s.pop(); }
                _ => ()
            }
        }
        _ => ()
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut s = String::from("");
    loop {
        update_string(&mut s);
        let tokens = lexic_analize(&s[..]);
        let mut styled_words = Vec::new();
        let mut errors = Vec::new();
        for x in tokens {
            match x {
                Number(_, _) => styled_words.push(x.to_string().magenta()),
                Operation(_, _) => styled_words.push(x.to_string().blue()),
                OpenParethesis(_) => styled_words.push(x.to_string().yellow()),
                CloseParethesis(_) => styled_words.push(x.to_string().yellow()),
                Keyword(_, _) => styled_words.push(x.to_string().green()),
                Error(pos, letter) => {
                    styled_words.push(x.to_string().red());
                    errors.push(Error(pos, letter))
                }
                ID(_, _) => styled_words.push(x.to_string().cyan()),
            }
        }
        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
        queue!(stdout, cursor::MoveTo(0, 0))?;
        for w in styled_words {
            queue!(stdout, style::PrintStyledContent(w))?;
        }

        queue!(stdout, cursor::MoveTo(0, 3))?;
        for err in errors {
            if let Error(pos, char) = err {
                if char != ' ' {
                    queue!(stdout, style::PrintStyledContent(format!("Error at pos {}, unkown character {}\n", pos, char).red()))?;
                }
            }
        }

        stdout.flush()?;
    }
}


// fn mutate_string(s :&mut String) {
//     s.push('a')
// }
//
// fn main() {
//     let mut s = String::from("");
//     while s.len() < 10 {
//         mutate_string(&mut s);
//         println!("{}", s);
//     }
//     return;
// }