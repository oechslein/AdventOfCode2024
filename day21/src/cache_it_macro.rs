#[macro_export]
macro_rules! cache_it {
    ($cache_type:ty, $cache_init:expr, $key:expr, $expression:expr) => {{
        // LazyLock to initialize a static Mutex<FxHashMap<String, usize>>
        // Mutex to lock the FxHashMap because of the Rayon parallel iterator
        static CACHE: std::sync::LazyLock<std::sync::RwLock<$cache_type>> =
            std::sync::LazyLock::new(|| std::sync::RwLock::new($cache_init));

        let key = ($key);
        // Lock the cache in this block
        {
            let cache = CACHE.read().unwrap();
            if let Some(value) = cache.get(&key) {
                return *value;
            }
        }

        let value = ($expression);

        // Lock the cache in this block
        {
            let mut cache = CACHE.write().unwrap();
            cache.insert(key, value);

            value
        }
    }};
}
