pub mod lru;
pub mod fifo;
pub mod lfu;

pub trait EvictionPolicy<K> {
    fn record_access(&mut self, key: &K);
    fn record_insertion(&mut self, key: &K);
    fn evict(&mut self) -> Option<K>;
}
