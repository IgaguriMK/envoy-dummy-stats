const BUCKETS_COUNT: usize = 20;
const LE_VALUES: [f64; BUCKETS_COUNT - 1] = [
    0.5, 1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 2500.0, 5000.0, 10000.0, 30000.0,
    60000.0, 300000.0, 600000.0, 1800000.0, 3600000.0,
];

#[derive(Debug, Default, Clone)]
pub struct Counter {
    count: usize,
    sum: f64,
    buckets: Buckets,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            count: 0,
            sum: 0.0,
            buckets: Buckets::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Buckets([usize; BUCKETS_COUNT]);

impl Buckets {
    fn new() -> Buckets {
        Buckets([0; BUCKETS_COUNT])
    }

    fn add(&mut self, val: f64) {
        for i in 0..BUCKETS_COUNT - 1 {
            if val < LE_VALUES[i] {
                self.0[i] += 1;
                return;
            }
        }
        self.0[BUCKETS_COUNT - 1] += 1;
    }
}
