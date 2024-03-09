struct Waiting {
    waiting_time: std::time::Duration,
}

struct Filling {
    rate: usize,
}

#[derive(Clone)]
struct Done;

struct BottleFillingMachine<S> {
    state: S,
    shared_value: usize
}

impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(value: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
        BottleFillingMachine{
            state: Filling {rate: 1},
            shared_value: value.shared_value
        }
    }
}


impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(value: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done>{
        BottleFillingMachine{
            state: Done,
            shared_value: value.shared_value
        }
    }
}

impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            state: Waiting {waiting_time: std::time::Duration::new(0, 0)},
            shared_value
        }
    }
}

// impl PartialEq for BottleFillingMachine<Waiting> {
//     fn eq(&self, other: &Self) -> bool {
//         self.state.waiting_time == other.state.waiting_time && self.shared_value == other.shared_value
//     }
// }
//
// impl PartialEq for BottleFillingMachine<Filling> {
//     fn eq(&self, other: &Self) -> bool {
//        self.state.rate == other.state.rate && self.shared_value == other.shared_value
//     }
// }
//
//
// impl PartialEq for BottleFillingMachine<Done> {
//     fn eq(&self, other: &Self) -> bool {
//         self.shared_value == other.shared_value && self.state == other.state
//     }
// }
//


enum BottleFillingMachineWrapper {
    Waiting(BottleFillingMachine<Waiting>),
    Filling(BottleFillingMachine<Filling>),
    Done(BottleFillingMachine<Done>)
}


struct MachineFactory {
    bottle_filling_machine: BottleFillingMachineWrapper
}

impl MachineFactory {
    fn new() -> Self {
        MachineFactory {
            bottle_filling_machine: BottleFillingMachineWrapper::Waiting(BottleFillingMachine::new(0))
        }
    }
}


impl BottleFillingMachineWrapper {
    fn step(mut self) -> Self {
        match self {
            BottleFillingMachineWrapper::Waiting(val) => BottleFillingMachineWrapper::Filling(val.into()),
            BottleFillingMachineWrapper::Filling(val) => BottleFillingMachineWrapper::Done(val.into()),
            BottleFillingMachineWrapper::Done(val) => BottleFillingMachineWrapper::Done(val)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_state() {
        let in_waiting = BottleFillingMachine::<Waiting>::new(1);
        let in_filling = BottleFillingMachine::<Filling>::from(in_waiting);
        let in_done = BottleFillingMachine::<Done>::from(in_filling);
    }


    #[test]
    fn state_factory() {
        let mut the_factory = MachineFactory::new();
        the_factory.bottle_filling_machine = the_factory.bottle_filling_machine.step();
    }
}