
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

mod enum_based_machine_state;

fn main() {
    let mut state_machine = enum_based_machine_state::StateMachine::new();
    state_machine = state_machine.to_filling();
    state_machine.current_state();
}
