#![feature(test)]
extern crate test;

extern crate rand;
extern crate mersenne_twister;


mod mt19937 {
    use rand::Rng;
    use mersenne_twister::MT19937;

    #[bench]
    fn benchmark_seeding(b: &mut ::test::Bencher) {
        b.iter(|| MT19937::new_unseeded());
    }

    #[bench]
    fn benchmark_fill_next_state(b: &mut ::test::Bencher) {
        let mt = MT19937::new_unseeded();
        b.iter(|| {
            let mut mt = mt.clone();
            // Note that the first call to next_u32() triggers a call
            // to the fill_next_state() method, which is really what I
            // want to benchmark here.
            mt.next_u32()
        })
    }
    
    #[bench]
    fn benchmark_gen_624_u32(b: &mut ::test::Bencher) {
        let mut mt = MT19937::new_unseeded();
        // Note that next_u32() periodically calls fill_next_state() every 624
        // uses, hence generate this many to get an average.
        b.iter(|| {
            let mut r = 0;
            for _ in 0..624 {
                r = mt.next_u32();
            }
            r   // return last value to prevent over-optimisation
        })
    }
    
    #[bench]
    fn benchmark_gen_312_u64(b: &mut ::test::Bencher) {
        let mut mt = MT19937::new_unseeded();
        // Note that next_u64() periodically calls fill_next_state() every 312
        // uses, hence generate this many to get an average.
        b.iter(|| {
            let mut r = 0;
            for _ in 0..312 {
                r = mt.next_u64();
            }
            r   // return last value to prevent over-optimisation
        })
    }
}


mod mt19937_64 {
    use rand::Rng;
    use mersenne_twister::MT19937_64;

    #[bench]
    fn benchmark_seeding(b: &mut ::test::Bencher) {
        b.iter(|| MT19937_64::new_unseeded());
    }

    #[bench]
    fn benchmark_fill_next_state(b: &mut ::test::Bencher) {
        let mt = MT19937_64::new_unseeded();
        b.iter(|| {
            let mut mt = mt.clone();
            // Note that the first call to next_u32() triggers a call
            // to the fill_next_state() method, which is really what I
            // want to benchmark here.
            mt.next_u64()
        })
    }
    
    #[bench]
    fn benchmark_gen_624_u32(b: &mut ::test::Bencher) {
        let mut mt = MT19937_64::new_unseeded();
        // Note that next_u32() periodically calls fill_next_state() every 312
        // uses. Use 624 to be equivalent to 32-bit implementation.
        b.iter(|| {
            let mut r = 0;
            for _ in 0..624 {
                r = mt.next_u32();
            }
            r   // return last value to prevent over-optimisation
        })
    }
    
    #[bench]
    fn benchmark_gen_312_u64(b: &mut ::test::Bencher) {
        let mut mt = MT19937_64::new_unseeded();
        // Note that next_u64() periodically calls fill_next_state() every 312
        // uses, hence generate this many to get an average.
        b.iter(|| {
            let mut r = 0;
            for _ in 0..312 {
                r = mt.next_u64();
            }
            r   // return last value to prevent over-optimisation
        })
    }
}
