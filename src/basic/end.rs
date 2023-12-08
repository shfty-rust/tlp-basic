use super::{Basic, Run};

/// Exit exits the program
pub struct End;

impl<Env> Basic<Env, End> {
    pub fn step(self) -> ! {
        std::process::exit(0)
    }
}

impl<Env> Run for Basic<Env, End> {
    fn run(self) {
        self.step()
    }
}
