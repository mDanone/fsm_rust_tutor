
// OUR STATE MACHINE MUST BE
// 1. Can only be in one state at a time
// 2. Each state should have its own associated values if required.
// 3. Transitioning between states should have well-defined semantics.
// 4. It should be possible to have some level of shared state.
// 5. Only explicitly defined transitions should be permitted
// 6. Changing from one state to another should consume the state, so it can no longer be used.
// 7. We shouldn't need to allocate memory for all states. No more than, largest sized state certainly
// 8. Any error messages should be easy to understand.
// 9. We shouldn't need to resort to heap allocations to do this. Everything should be possible on the stack.
// 10. The type system should be harnessed to our greatest ability.
// 11. As many errors as possible should be at compile-time


#[derive(Debug)]
enum BottleFillerState {
    Waiting {waiting_time: std::time::Duration},
    Filling {rate: usize},
    Done
}


struct StateMachine {
    state: BottleFillerState
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: BottleFillerState::Waiting {waiting_time: std::time::Duration::new(0, 0)}
        }
    }

    fn to_filling(mut self) -> StateMachine {
        self.state = match self.state {
            BottleFillerState::Waiting { .. } => BottleFillerState::Filling {rate: 1},
            _ => panic!("Invalid state transition!"),
        };
        self
    }
}

fn main() {
    let mut state_machine = StateMachine::new();
    state_machine = state_machine.to_filling();
    println!("{:?}", state_machine.state);
}
