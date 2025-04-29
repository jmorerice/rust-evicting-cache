use rusty_cache::{Cache, LruPolicy, FifoPolicy};

fn main() {
    println!("Testing LRU Cache:");
    test_lru();

    println!("\nTesting FIFO Cache:");
    test_fifo();

    println!("\nTesting LFU Cache:");
    //test_lfu();
}

fn test_lru() {
    let mut cache = Cache::new(2, LruPolicy::new());

    cache.put("a", 1);
    cache.put("b", 2);

    // Access 'a' to make it recently used
    println!("{:?}", cache.get(&"a")); // Some(1)

    cache.put("c", 3); // Should evict "b"

    println!("{:?}", cache.get(&"b")); // None
    println!("{:?}", cache.get(&"a")); // Some(1)
    println!("{:?}", cache.get(&"c")); // Some(3)
}

fn test_fifo() {
    let mut cache = Cache::new(2, FifoPolicy::new());

    cache.put("a", 1);
    cache.put("b", 2);

    // No access pattern matters in FIFO
    println!("{:?}", cache.get(&"a")); // Some(1)
    println!("{:?}", cache.get(&"b")); // Some(2)

    cache.put("c", 3); // Should evict "a" (first inserted)

    // Check evictions
    println!("{:?}", cache.get(&"a")); // None
    println!("{:?}", cache.get(&"b")); // Some(2)
    println!("{:?}", cache.get(&"c")); // Some(3)
}

// fn test_lfu() {
//     let mut cache = Cache::new(2, LfuPolicy::new());

//     cache.put("a", 1);
//     cache.put("b", 2);

//     // Access 'a' twice to make it more frequent
//     println!("{:?}", cache.get(&"a"));
//     println!("{:?}", cache.get(&"a"));

//     cache.put("c", 3); // Should evict "b" (less frequently used)

//     // Check evictions
//     println!("{:?}", cache.get(&"b")); // None
//     println!("{:?}", cache.get(&"a")); // Some(1)
//     println!("{:?}", cache.get(&"c")); // Some(3)
// }