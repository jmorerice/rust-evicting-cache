# rust-evicting-cache

🦀 A bounded in-memory cache library written in Rust with pluggable eviction policies (LRU, FIFO, LFU), time-to-live (TTL) support, and thread safety.

---

## 🚀 Project Goals

This project explores key systems programming concepts through the implementation of a production-ready in-memory cache:

- 🔁 **Eviction Policies**: Support for LRU, FIFO, LFU (pluggable strategy pattern)
- 🧠 **Custom Bounded Size**: Based on item count (memory bounds optional later)
- ⏱️ **TTL Support**: Optional per-key expiration with cleanup
- 🔐 **Thread-Safety**: Safe access across threads using `Mutex` or `RwLock`
- 📦 **Extensible Design**: Easy to add logging hooks or external storage fallback

---

## 📦 Features

- ✅ Simple API: `put`, `get`, `evict`, `clear`
- ⚙️ Configurable capacity
- 🔁 Swappable eviction policy via traits
- ⏲️ Optional TTL expiration
- 🧵 Thread-safe variant using `dashmap` or manual locking
- 🪵 Eviction hooks (e.g., logging or callback support)

---

## 🧠 Why This Exists

This project was built as a learning exercise to:
- Understand memory safety, ownership, and concurrency in Rust
- Gain experience with systems design patterns
- Prepare for interviews at high-performance computing or trading firms (e.g., Jane Street, HFTs)
- Reinforce core concepts around cache design and real-world performance tradeoffs

---

## ✨ Usage Example

```rust
use rust_evicting_cache::Cache;

fn main() {
    let mut cache = Cache::new(3);

    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);

    println!("{:?}", cache.get(&"a")); // Some(1)

    cache.put("d", 4); // Triggers eviction (if over capacity)

    println!("{:?}", cache.get(&"b")); // Might be None if evicted
}
