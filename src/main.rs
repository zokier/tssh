extern crate readline;

enum EvalResult {
    Ok(String),
    Err(String),
    Cont
}

fn eval_exec(cmd: &str) -> EvalResult {
    return EvalResult::Ok(cmd.to_string());
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
        let res = match readline::readline(prompt) {
            Ok(line) => {
                match eval(&line, &mut cmd_buf) {
                    EvalResult::Cont => { 
                        prompt = "> ";
                        continue
                    },
                    EvalResult::Ok(output) => {
                        prompt = "$ ";
                        output
                    },
                    EvalResult::Err(e) => {
                        println!("tssh error: {}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                match e {
                    readline::ReadlineError::EndOfFile => println!(""),
                    readline::ReadlineError::InvalidUtf8(utf8_err) => println!("{:?}", utf8_err)
                };
                return;
            }
        };
        println!("{}", res);
    };
}
