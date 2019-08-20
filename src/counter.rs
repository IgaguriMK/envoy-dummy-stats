use std::sync::{Arc, Mutex};

use gotham_derive::StateData;

const BUCKETS_COUNT: usize = 20;
const LE_VALUES: [f64; BUCKETS_COUNT] = [
    0.5,
    1.0,
    5.0,
    10.0,
    25.0,
    50.0,
    100.0,
    250.0,
    500.0,
    1_000.0,
    2_500.0,
    5_000.0,
    10_000.0,
    30_000.0,
    60_000.0,
    300_000.0,
    600_000.0,
    1_800_000.0,
    3_600_000.0,
    std::f64::INFINITY,
];

#[derive(Debug, Default, Clone, StateData)]
pub struct Counter {
    inner: Arc<Mutex<CounterState>>,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            inner: Arc::new(Mutex::new(CounterState::new())),
        }
    }

    pub fn add(&self, val: f64) {
        let mut state = self.inner.as_ref().lock().unwrap();
        state.add(val);
    }

    pub fn get_count(&self) -> CounterState {
        let state = self.inner.as_ref().lock().unwrap();
        state.clone()
    }
}

#[derive(Debug, Default, Clone)]
pub struct CounterState {
    count: usize,
    sum: f64,
    buckets: Buckets,
}

impl CounterState {
    fn new() -> CounterState {
        CounterState {
            count: 0,
            sum: 0.0,
            buckets: Buckets::new(),
        }
    }

    fn add(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;
        self.buckets.add(val);
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn buckets(&self) -> BucketsIter {
        BucketsIter {
            buckets: &self.buckets,
            sum: 0,
            idx: 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Buckets([usize; BUCKETS_COUNT]);

impl Buckets {
    fn new() -> Buckets {
        Buckets([0; BUCKETS_COUNT])
    }

    fn add(&mut self, val: f64) {
        for (i, &le) in LE_VALUES.iter().enumerate() {
            if val < le {
                self.0[i] += 1;
                return;
            }
        }
        self.0[BUCKETS_COUNT - 1] += 1;
    }
}

#[derive(Debug)]
pub struct BucketsIter<'a> {
    buckets: &'a Buckets,
    sum: usize,
    idx: usize,
}

impl<'a> Iterator for BucketsIter<'a> {
    type Item = BucketCount;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < BUCKETS_COUNT {
            let le = LE_VALUES[self.idx];
            let count = self.buckets.0[self.idx];
            self.sum += count;

            self.idx += 1;

            Some(BucketCount {
                le,
                count,
                sum: self.sum,
            })
        } else {
            None
        }
    }
}

pub struct BucketCount {
    pub le: f64,
    pub count: usize,
    pub sum: usize,
}
