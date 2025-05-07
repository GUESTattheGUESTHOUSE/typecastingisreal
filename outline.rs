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

////this works so far but i can do stuff like create fuctions that
////do most of the work instead of having to verbose in main
///idk just old habits form asm nothing global
 /* let mut pool: Vec<(u8, bool)> = vec![(1, true),
        (2, false),
        (3, false),
        (4, false),
        (5, false)];
    
    pool.push((6, true));
    
    println!("{:?}", pool);
*/
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

//its own thing tyring to refine stuff, just like fine wine or a nice data
//i mean i could go one you would like that wouldnt you
fn main() {
    enum Poolsizeoptions {
        Byte(u8),
        Double(u16),
        Quad(u32),
        Oct(u64),
    }

    struct Randdata{
        poolsize: Poolsizeoptions::byte,
        rngpool: Vec<(pool, bool)>,
    }
    
/*    impl randata {
        fn initpools (numberofpools: u8, size: poolsize) -> Self{
            for numberofpools {
                rngpool.push((poolsize, true));
            }
        }
        fn pullfromrngthread {
            
        }
    }
*/

    let mut pool: Randdata = Randdata {
        poolsize: 5u8, //i think i need to match but probably through an impl fn
        rngpool: (3u8, true)
    };
}
