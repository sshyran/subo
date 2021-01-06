use suborbital::runnable;

struct Helloworld{}

impl runnable::Runnable for Helloworld {
    fn run(&self, input: Vec<u8>) -> Option<Vec<u8>> {
        let in_string = String::from_utf8(input).unwrap();
    
        Some(String::from(format!("hello {}", in_string)).as_bytes().to_vec())
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &Helloworld = &Helloworld{};

#[no_mangle]
pub extern fn init() {
    runnable::set(RUNNABLE);
}
