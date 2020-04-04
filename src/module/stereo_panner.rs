use crate::engine::{Sample, ZERO_BUFFER_MONO};
use crate::module::{Module, LineType};

#[derive(Debug)]
pub struct StereoPanner;

impl Module for StereoPanner {
    type Params = ();
    type Indication = ();

    fn create(_: Self::Params) -> (Self, Self::Indication) {
        (StereoPanner, ())
    }

    fn params(&self) -> Self::Params {
        ()
    }

    fn update(&mut self, _: Self::Params) -> Option<Self::Indication> {
        None
    }

    fn run_tick(&mut self, _t: u64, inputs: &[Option<&[Sample]>], outputs: &mut [&mut [Sample]]) -> Option<Self::Indication> {
        let left = &inputs[0].unwrap_or(&ZERO_BUFFER_MONO);
        let right = &inputs[1].unwrap_or(&ZERO_BUFFER_MONO);
        let output = &mut outputs[0];

        for i in 0..left.len() {
            output[i * 2 + 0] = left[i];
            output[i * 2 + 1] = right[i];
        }

        None
    }

    fn inputs(&self) -> &[LineType] {
        &[LineType::Mono, LineType::Mono]
    }

    fn outputs(&self) -> &[LineType] {
        &[LineType::Stereo]
    }
}
