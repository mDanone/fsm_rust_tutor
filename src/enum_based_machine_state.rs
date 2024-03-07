#[derive(Debug)]
pub enum BottleFillerState {
    Waiting {waiting_time: std::time::Duration},
    Filling {rate: usize},
    Done
}


pub struct StateMachine {
    state: BottleFillerState
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: BottleFillerState::Waiting {waiting_time: std::time::Duration::new(0, 0)}
        }
    }

    pub fn to_filling(mut self) -> StateMachine {
        self.state = match self.state {
            BottleFillerState::Waiting { .. } => BottleFillerState::Filling {rate: 1},
            _ => panic!("Invalid state transition!"),
        };
        self
    }

    pub fn current_state(&self) -> &BottleFillerState {
        println!("{:?}", &self.state);
        &self.state
    }
}