// pub trait Input {
//     fn next(&mut self) -> i32;
//     fn modify(&mut self, modify: i32);
// }

// pub struct RepeatInput {
//     rep: i32,
// }

// impl RepeatInput {
//     fn new(repeat: i32) -> Self {
//         RepeatInput {
//             rep: repeat,
//         }
//     }
// }

// impl Input for RepeatInput {
//     fn next(&mut self) -> i32 {
//         self.rep
//     }
//     fn modify(&mut self, modification: i32) {
//         self.rep = modification;
//     }
// }

// pub struct RepeatInputs {
//     idx: usize,
//     inputs: Vec<i32>,
// }

// impl RepeatInputs {
//     fn new(repeat: Vec<i32>) -> Self {
//         RepeatInputs {
//             idx: 0,
//             inputs: repeat,
//         }
//     }
// }

// impl Input for RepeatInputs {
//     fn next(&mut self) -> i32 {
//         let res = self.inputs[self.idx];
//         self.idx = (self.idx + 1) % self.inputs.len();
//         res
//     }
//     fn modify(modify: i32) {

//     }
// }