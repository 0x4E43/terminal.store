extern crate termion;

use termion::input::TermRead;
use std::io::{self, stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;
use termion::{color, style, clear, cursor};

fn main() {
    // Setup terminal in raw mode
    let stdin = io::stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print_heading(&mut stdout, 's');
    // Listen to keystrokes and print the key in real-time
    for key in stdin {
        match key {
            Ok(key) => {
                // Print pressed key
                write!(stdout, "\rPressed key: {:?}       \r\n", key).unwrap();
                stdout.flush().unwrap();

                // show item if s is pressed
                if let termion::event::Key::Char('s') = key {

                    print_heading(&mut stdout, 's');
                }
                // show about if a is pressed
                if let termion::event::Key::Char('a') = key {

                    print_heading(&mut stdout, 'a');
                }
                // show cart if c is pressed
                if let termion::event::Key::Char('c') = key {

                    print_heading(&mut stdout, 'c');
                }

                // show cart if c is pressed
                if let termion::event::Key::Char('d') = key {

                    print_heading(&mut stdout, 'd');
                }

                // Terminate loop if 'q' is pressed
                if let termion::event::Key::Char('q') = key {
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}



fn print_heading( stdout: &mut impl Write, selction : char){
    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "\r---------------------------------------------------\n").unwrap();
    write!(stdout, "\r|                  {}{}Hello World{} !{}{}                  |\n", style::Bold,color::Fg(color::Blue), color::Fg(color::Red),style::Blink, style::Reset).unwrap();
    write!(stdout, "\r---------------------------------------------------\n").unwrap();

    //menu 
    write!(stdout, "\r\n {}s {}shop |", color::Fg(color::Rgb(255,127,80)), color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "{} a {}about |", color::Fg(color::Rgb(255,127,80)), color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "{} c {}cart |", color::Fg(color::Rgb(255,127,80)), color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "{} d {}done", color::Fg(color::Rgb(255,127,80)), color::Fg(color::Rgb(169,169,169))).unwrap();
    
    if selction == 'c'{
        show_cart(stdout)
    }
    if selction =='s'{
        show_item(stdout);
    }
    if selction =='a'{
        show_about(stdout);
    }

    if selction =='d' {
        show_done(stdout);
    }
    

}

fn show_item(stdout: &mut impl Write){
    write!(stdout, "\r\n\n\n {}{}{}nil blend cofee{}\n", style::Underline, style::Bold, color::Fg(color::Rgb(255,0,0)), style::Reset).unwrap();
    write!(stdout, "\r\n{} whole bean | medium roast | 12oz\n",color::Fg(color::Rgb(169, 169, 169))).unwrap();
    write!(stdout, "\r\n{} Dive into the rich taste of Nil, our delicious semi-sweet", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} coffee with notes of chocolate, peanut butter, and a hint", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} of fig. Born in the lush expanses of Fazenda Rainha, a", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} 280-hectare coffee kingdom nestled in Brazil's Vale da", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} Grama. This isn't just any land; it's a legendary", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} volcanic valley, perfectly poised on the mystical borders", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} between São Paulo State and Minas Gerais. On the edge of", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} the Mogiana realm, Fazenda Rainha reigns supreme, a true", color::Fg(color::Rgb(169,169,169))).unwrap();
    write!(stdout, "\r\n{} coffee royalty crafting your next unforgettable cup.{}", color::Fg(color::Rgb(169,169,169)), style::Reset).unwrap();



    write!(stdout, "\r\n\n Press [q] to QUIT {}\r", cursor::Hide).unwrap();
    stdout.flush().unwrap();
}

fn show_cart(stdout: &mut impl Write){
    write!(stdout, "\r\n\n\n {}{}{}Hello Cart{}\n", style::Underline, style::Bold, color::Fg(color::White), style::Reset).unwrap();
    write!(stdout, "\r\n\n Press [q] to QUIT {}\r", cursor::Hide).unwrap();
    stdout.flush().unwrap();
}


fn show_about(stdout: &mut impl Write){
    write!(stdout, "\r\n\n{}{}Hello Terminal user{}\n", style::Bold, color::Fg(color::White), style::Reset).unwrap();

    write!(stdout, "\r\n\n Trying to make something awesome with the power of terminal\n",).unwrap();
    write!(stdout, "\r\n\n 1. # @0x4E43 [Nimai]\r").unwrap();

    write!(stdout, "\r\n\n Press [q] to QUIT {}\r", cursor::Hide).unwrap();
    stdout.flush().unwrap();
}


fn show_done(stdout: &mut impl Write){
    write!(stdout, "\r\n\n{}{}Hello Terminal user{}\n", style::Bold, color::Fg(color::White), style::Reset).unwrap();
    let mut s=String::new();
    write!(stdout,"\rPlease enter some text: {}", cursor::Restore ).unwrap();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    // if let Some('\n')=s.chars().next_back() {
        //     s.pop();
        // }
        // if let Some('\r')=s.chars().next_back() {
            //     s.pop();
            // }
            println!("You typed: {}",s);
            stdout.flush().unwrap();
        }
