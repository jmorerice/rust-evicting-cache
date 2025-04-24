use rusty_cache::Cache;
use rusty_cache::LruPolicy;

fn main() {
    let mut cache = Cache::new(2, LruPolicy::new());

    cache.put("a", 1);
    cache.put("b", 2);

    println!("{:?}", cache.get(&"a")); // Should be Some(1)
    
    cache.put("c", 3); // Evicts "b" (if "a" was accessed)

    println!("{:?}", cache.get(&"b")); // Should be None (evicted)
    println!("{:?}", cache.get(&"a")); // Should be Some(1)
    println!("{:?}", cache.get(&"c")); // Should be Some(3)
}
