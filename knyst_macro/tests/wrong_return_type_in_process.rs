// Generate code that doesn't compile, the Gen will not be used in any way

#[allow(unused)]
use knyst_macro::gen;

struct Sine {
    phase: f32,
}
#[gen]
impl Sine {
    #[process]
    fn process(&mut self, freq: &[Sample], phase: &[Sample], out0: &mut [Sample]) -> f32 {
        todo!()
    }
}

fn main() {}
