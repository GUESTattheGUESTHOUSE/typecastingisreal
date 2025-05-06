use rand_chacha; // 0.9.0a

//i hate this struct thing i want abitray size and number of pools but arbitry 
//is a bad way to put thingsm, pool size should be byte size. and numbers
//idk check systems to memory limitations. so just making it fixed for the time
//may a vec for number of pool and an enum for size idk. primatives are confusing
//all this is just to make sure nobody and take too much, ideally pool are sized
//for the request and if an bad actor tries to dos the pool and create a weak
//rng situation then with the groups and the thread can like counter that
//its closer to game theory than straight cryptography, but yeah 5 works at like
//idk a byte, i should do more, but i dont know how much i need, and too much
//is fucking way to hard to tell if it is bad. but better than not enough

struct rng_pool {
    p1: u8, //add another field that is a bool to signify what is empty and what is full
    p2: u8,
    p3: u8,
    p4: u8,
    p5: u8,
}

impl rng_pool {
    
    //create the pool/variable/space in memory
    fn init_pool() -> Self {
        Self {
            p1: ChaCha12Rng::from_os_rng(), // when init it will mark full
            p2: ChaCha12Rng::from_os_rng(),
            p3: ChaCha12Rng::from_os_rng(),
            p4: ChaCha12Rng::from_os_rng(),
            p5: ChaCha12Rng::from_os_rng(),
        }
    }
    
    //fill the empty stuff up
    fn refill_pool() -> Self {
    }
    
    //use pool
    fn use_data() {
        //pick a number 1 - 5 and use for
        //rng
    }
}
