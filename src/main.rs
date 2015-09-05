extern crate readline;

enum EvalResult {
    Ok,
    Err(String),
    Cont
}

#[derive(Copy,Clone)]
enum ParseState {
    Quoted,
    BackslashQuoted,
    EndQuote,
    Normal,
    BackslashNormal,
}

fn parse(line: &str) -> Result<Vec<String>, String> {
    let mut res = Vec::<String>::new();
    let mut current_token = String::new();
    let mut state = ParseState::Normal;
    for c in line.chars() {
        state = match (c, state) {
            (' ', ParseState::Quoted) => {
                current_token.push(' ');
                state
            },
            (' ', ParseState::BackslashQuoted) => {
                current_token.push('\\');
                current_token.push(' ');
                ParseState::Quoted
            },
            (' ', ParseState::EndQuote) => {
                res.push(current_token.clone());
                current_token.clear();
                ParseState::Normal
            },
            (_, ParseState::EndQuote) => {
                return Err("expected space after \"".to_string());
            },
            (' ', ParseState::BackslashNormal) => {
                current_token.push(' ');
                ParseState::Normal
            },
            (' ', ParseState::Normal) => {
                res.push(current_token.clone());
                current_token.clear();
                state
            },
            ('"', ParseState::Quoted) => {
                ParseState::EndQuote
            },
            ('"', ParseState::BackslashQuoted) => {
                current_token.push('"');
                ParseState::Quoted
            },
            ('"', ParseState::BackslashNormal) => {
                current_token.push('"');
                ParseState::Normal
            },
            ('"', ParseState::Normal) => {
                ParseState::Quoted
            },
            ('\\', ParseState::Quoted) => {
                ParseState::BackslashQuoted
            },
            ('\\', ParseState::BackslashQuoted) => {
                current_token.push('\\');
                ParseState::Quoted
            },
            ('\\', ParseState::BackslashNormal) => {
                current_token.push('\\');
                ParseState::Normal
            },
            ('\\', ParseState::Normal) => {
                ParseState::BackslashNormal
            },
            (c, ParseState::BackslashQuoted) => {
                current_token.push('\\');
                current_token.push(c);
                ParseState::Quoted
            },
            (_c, ParseState::BackslashNormal) => {
                //TODO some fancy escape codes, eg \n
                ParseState::Normal
            },
            (c, _) => {
                current_token.push(c);
                state
            }
        }
    }
    match state {
        ParseState::Normal => {
            res.push(current_token);
        },
        ParseState::EndQuote => {
            res.push(current_token);
        },
        _ => {
            return Err("Unexpected EOL".to_string());
        }
    }

    return Ok(res);
}

fn eval_exec(cmd: &str) -> EvalResult {
    return 
        match parse(cmd) {
            Ok(parsed) => { 
                println!("{:?}", parsed); 
                EvalResult::Ok
            }
            Err(err) => EvalResult::Err(err)
        };
}

fn eval(line: &str, cmd_buf: &mut String) -> EvalResult {
    cmd_buf.push_str(&line);

    if cmd_buf.ends_with("\\") {
        cmd_buf.pop();
        return EvalResult::Cont;
    }

    readline::add_history(&cmd_buf);
    let res = eval_exec(&cmd_buf);
    cmd_buf.clear();
    return res;
}

fn main() {
    let mut cmd_buf = String::new();
    let mut prompt = "$ ";
    loop {
        match readline::readline(prompt) {
            Ok(line) => {
                match eval(&line, &mut cmd_buf) {
                    EvalResult::Cont => { 
                        prompt = "> ";
                    },
                    EvalResult::Ok => {
                        prompt = "$ ";
                    },
                    EvalResult::Err(e) => {
                        println!("tssh error: {}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                match e {
                    readline::ReadlineError::EndOfFile => println!("Bye!"),
                    readline::ReadlineError::InvalidUtf8(utf8_err) => println!("tssh error: invalid utf8: {}", utf8_err)
                };
                return;
            }
        };
    };
}
