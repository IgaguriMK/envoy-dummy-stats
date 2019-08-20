use std::thread;
use std::time::Duration;

use rand::{thread_rng, Rng};
use rand_distr::{Exp, Normal};

use crate::counter::Counter;
use crate::err::Error;

pub fn start(generator: Generator, counter: Counter) {
    thread::spawn(move || {
        let mut rng = thread_rng();
        loop {
            generator.wait(&mut rng);
            let val = generator.gen(&mut rng);
            counter.add(val);
        }
    });
}

pub struct Generator {
    distr: Normal<f64>,
    dur: Exp<f64>,
}

impl Generator {
    pub fn new(rate: f64, mean: f64, std_dev: f64) -> Result<Generator, Error> {
        Ok(Generator {
            distr: Normal::new(mean, std_dev)?,
            dur: Exp::new(rate)?,
        })
    }

    fn wait<R: Rng>(&self, rng: &mut R) {
        let dur_sec = rng.sample(self.dur);
        thread::sleep(Duration::from_micros((1_000_000.0 * dur_sec) as u64));
    }

    fn gen<R: Rng>(&self, rng: &mut R) -> f64 {
        loop {
            let v = rng.sample(self.distr);
            if v > 0.0 {
                return v;
            }
        }
    }
}
