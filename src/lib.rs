// A base in-memory cache library with bounded capacity.
// Future support will include eviction policies, TTL expiration, and thread safety.

pub mod cache;
pub mod eviction;

pub use cache::Cache;
pub use eviction::{EvictionPolicy, lru::LruPolicy};