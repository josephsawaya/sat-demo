use suborbital::http::get;
use suborbital::runnable::*;

struct HelloEcho {}

impl Runnable for HelloEcho {
    fn run(&self, _input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        // let in_string = String::from_utf8(input).unwrap();
        let res = get("https://jsonplaceholder.typicode.com/posts/1", None);
        let json = match res {
            Ok(temp) => String::from_utf8(temp).unwrap(),
            Err(err) => {
                return Err(RunErr {
                    code: 500,
                    message: err.message,
                })
            }
        };

        Ok(format!("{}", json).as_bytes().to_vec())
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &HelloEcho = &HelloEcho {};

#[no_mangle]
pub extern "C" fn _start() {
    use_runnable(RUNNABLE);
}
