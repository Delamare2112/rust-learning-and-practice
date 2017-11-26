pub trait IndexOrInsert<K, V> {
    fn ioi(&mut self, key: K) -> &mut V;
}

#[allow(unused_macros)]
macro_rules! impl_trev_lib {
    () => {
        impl<K, V> IndexOrInsert<K, V> for ::std::collections::HashMap<K, V> where V: Default, K: Eq + ::std::hash::Hash {
            fn ioi(&mut self, k: K) -> &mut V {
                self.entry(k).or_insert(V::default())
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! import {
    ($m:ident, $p:expr) => {
        #[path=$p]
        mod $m;
    }
}

#[allow(unused_macros)]
macro_rules! import_all {
    ($m:ident, $p:expr) => {
        #[path=$p]
        mod $m;
        use self::$m::*;
    }
}
