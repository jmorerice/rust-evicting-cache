# rust-evicting-cache

🦀 A bounded, in-memory cache written in Rust with configurable eviction policies (LRU, FIFO, LFU), optional TTL expiration, and thread safety.

---

## 📦 Features

- 💻 Simple API: `put`, `get`, `evict`, `clear`
- 🔁 **Eviction Policies**: Pluggable strategies (LRU, FIFO, LFU)
- ⚙️ **Custom Bounded Size**: Limit cache by item count (or memory usage, optional)
- ⏱️ **TTL Support**: Optional time-to-live expiration per entry
- 🧵 + 🔐 **Thread-Safety**: Safe across threads and concurrent access using locking 
- 🪵 **Eviction Hooks**: Optional callbacks/logging for evicted items
- 🧱 **Extensible**: Easily add metrics, serialization, or fallback to external storage

---

## 💡 Usage Example

```rust
use rust_evicting_cache::Cache;

fn main() {
    let mut cache = Cache::new(3);

    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);

    println!("{:?}", cache.get(&"a")); // Some(1)

    cache.put("d", 4); // Triggers eviction if capacity exceeded

    println!("{:?}", cache.get(&"b")); // May be None if evicted
}
```

---

## 🔧 Build & Run
```rust
git clone https://github.com/your-username/rust-evicting-cache
cd rust-evicting-cache
cargo run       # if you include a main.rs
cargo test      # to run unit tests
```
---

## 🗺️ Roadmap
- Base cache with fixed capacity ✅
- Add LRU policy
- Add FIFO / LFU policy
- TTL support with background cleanup
- Eviction callbacks/logging
- Thread-safe implementation
- Optional benchmarking

---
## 🧑‍💻 Built With
- Rust
- HashMap, RwLock, dashmap, chrono
- Testing via cargo test

---
## 📘 Summary
This project demonstrates key systems programming concepts in Rust, including:

- Ownership and borrowing
- Concurrency and locking
- Trait-based design patterns
- TTL expiration and cleanup logic

Originally inspired by cache problems discussed in distributed systems, backend engineering, and performance-critical settings.
