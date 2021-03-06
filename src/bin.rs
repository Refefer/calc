extern crate calc;
extern crate clap;
extern crate liner;

use std::fmt;
use std::process::exit;

use std::io::{self, stdout, Write};

use calc::{eval, eval_polish, CalcError};

use clap::{Arg, App};

use liner::Context;

const PROMPT: &'static str = "[]> ";

pub fn prompt<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "{}", PROMPT)?;
    out.flush()
}

pub enum RuntimeError {
    Calc(CalcError),
    IO(io::Error),
}

impl From<CalcError> for RuntimeError {
    fn from(data: CalcError) -> RuntimeError {
        RuntimeError::Calc(data)
    }
}

impl From<io::Error> for RuntimeError {
    fn from(data: io::Error) -> RuntimeError {
        RuntimeError::IO(data)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeError::Calc(ref c) => write!(f, "calc: {}", c),
            RuntimeError::IO(ref e) => write!(f, "calc: {}", e),
        }
    }
}

pub fn calc() -> Result<(), RuntimeError> {
    let stdout = stdout();
    let mut stdout = stdout.lock();

    let matches = App::new("calc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Hunter Goldstein <hunter.d.goldstein@gmail.com>")
        .about("Floating point calculator")
        .set_term_width(80)
        .arg(Arg::with_name("polish")
             .short("p")
             .long("polish")
             .help("Parse expressions using polish notation versus infix notation"))
        .arg(Arg::with_name("expr")
             .help("Expression to evaluate by this program. If this argument is missing, enter interactive mode.")
             .multiple(true)
             .value_name("EXPR"))
        .get_matches();

    // Check if the polish notation flag was given.
    let polish = matches.is_present("polish");

    match matches.values_of("expr") {
        Some(values) => {
            writeln!(
                stdout,
                "{}",
                if polish {
                    eval_polish(&values.fold(String::new(), |acc, s| acc + s))?
                } else {
                    eval(&values.fold(String::new(), |acc, s| acc + s))?
                }
            )?;
        }
        None => {
            let mut con = Context::new();
            loop {
                let line = con.read_line(PROMPT, &mut |_| {})?;
                match line.trim() {
                    "" => (),
                    "exit" => break,
                    s => {
                        writeln!(
                            stdout,
                            "{}",
                            if polish { eval_polish(s)? } else { eval(s)? }
                        )?
                    }
                }
                con.history.push(line.into())?;
            }
        }
    }
    Ok(())
}

fn main() {
    let code = match calc() {
        Ok(()) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    };
    exit(code)
}
