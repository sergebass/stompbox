use crate::effects::SampleProcessor;

pub struct WhiteNoiseGenerator<T>
where
    T: random::Source,
{
    random_source: T,
}

impl<T> WhiteNoiseGenerator<T>
where
    T: random::Source,
{
    #[inline(always)]
    pub fn new(t: T) -> Self {
        Self { random_source: t }
    }
}

impl<T> SampleProcessor for WhiteNoiseGenerator<T>
where
    T: random::Source,
{
    fn name(&self) -> &str {
        "White Noise"
    }

    fn process_sample(&mut self, _input_sample: f32) -> f32 {
        self.random_source.read::<f32>()
    }
}
