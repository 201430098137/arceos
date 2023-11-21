// #[cfg(test)]
// mod tests;
//
// use self::Entry::*;
//
use super::hashbrown as base;
//
// use crate::borrow::Borrow;
// use crate::collections::TryReserveError;
// use crate::collections::TryReserveErrorKind;
// use crate::error::Error;
// use crate::fmt::{self, Debug};
use super::hash::{BuildHasher, Hash, RandomState};
// use crate::iter::FusedIterator;
// use crate::ops::Index;
//
// /// A [hash map] implemented with quadratic probing and SIMD lookup.
// ///
// /// By default, `HashMap` uses a hashing algorithm selected to provide
// /// resistance against HashDoS attacks. The algorithm is randomly seeded, and a
// /// reasonable best-effort is made to generate this seed from a high quality,
// /// secure source of randomness provided by the host without blocking the
// /// program. Because of this, the randomness of the seed depends on the output
// /// quality of the system's random number coroutine when the seed is created.
// /// In particular, seeds generated when the system's entropy pool is abnormally
// /// low such as during system boot may be of a lower quality.
// ///
// /// The default hashing algorithm is currently SipHash 1-3, though this is
// /// subject to change at any point in the future. While its performance is very
// /// competitive for medium sized keys, other hashing algorithms will outperform
// /// it for small keys such as integers as well as large keys such as long
// /// strings, though those algorithms will typically *not* protect against
// /// attacks such as HashDoS.
// ///
// /// The hashing algorithm can be replaced on a per-`HashMap` basis using the
// /// [`default`], [`with_hasher`], and [`with_capacity_and_hasher`] methods.
// /// There are many alternative [hashing algorithms available on crates.io].
// ///
// /// It is required that the keys implement the [`Eq`] and [`Hash`] traits, although
// /// this can frequently be achieved by using `#[derive(PartialEq, Eq, Hash)]`.
// /// If you implement these yourself, it is important that the following
// /// property holds:
// ///
// /// ```text
// /// k1 == k2 -> hash(k1) == hash(k2)
// /// ```
// ///
// /// In other words, if two keys are equal, their hashes must be equal.
// /// Violating this property is a logic error.
// ///
// /// It is also a logic error for a key to be modified in such a way that the key's
// /// hash, as determined by the [`Hash`] trait, or its equality, as determined by
// /// the [`Eq`] trait, changes while it is in the map. This is normally only
// /// possible through [`Cell`], [`RefCell`], global state, I/O, or unsafe code.
// ///
// /// The behavior resulting from either logic error is not specified, but will
// /// be encapsulated to the `HashMap` that observed the logic error and not
// /// result in undefined behavior. This could include panics, incorrect results,
// /// aborts, memory leaks, and non-termination.
// ///
// /// The hash table implementation is a Rust port of Google's [SwissTable].
// /// The original C++ version of SwissTable can be found [here], and this
// /// [CppCon talk] gives an overview of how the algorithm works.
// ///
// /// [hash map]: crate::collections#use-a-hashmap-when
// /// [hashing algorithms available on crates.io]: https://crates.io/keywords/hasher
// /// [SwissTable]: https://abseil.io/blog/20180927-swisstables
// /// [here]: https://github.com/abseil/abseil-cpp/blob/master/absl/container/internal/raw_hash_set.h
// /// [CppCon talk]: https://www.youtube.com/watch?v=ncHmEUmJZf4
// ///
// /// # Examples
// ///
// /// ```
// /// use std::collections::HashMap;
// ///
// /// // Type inference lets us omit an explicit type signature (which
// /// // would be `HashMap<String, String>` in this example).
// /// let mut book_reviews = HashMap::new();
// ///
// /// // Review some books.
// /// book_reviews.insert(
// ///     "Adventures of Huckleberry Finn".to_string(),
// ///     "My favorite book.".to_string(),
// /// );
// /// book_reviews.insert(
// ///     "Grimms' Fairy Tales".to_string(),
// ///     "Masterpiece.".to_string(),
// /// );
// /// book_reviews.insert(
// ///     "Pride and Prejudice".to_string(),
// ///     "Very enjoyable.".to_string(),
// /// );
// /// book_reviews.insert(
// ///     "The Adventures of Sherlock Holmes".to_string(),
// ///     "Eye lyked it alot.".to_string(),
// /// );
// ///
// /// // Check for a specific one.
// /// // When collections store owned values (String), they can still be
// /// // queried using references (&str).
// /// if !book_reviews.contains_key("Les Misérables") {
// ///     println!("We've got {} reviews, but Les Misérables ain't one.",
// ///              book_reviews.len());
// /// }
// ///
// /// // oops, this review has a lot of spelling mistakes, let's delete it.
// /// book_reviews.remove("The Adventures of Sherlock Holmes");
// ///
// /// // Look up the values associated with some keys.
// /// let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
// /// for &book in &to_find {
// ///     match book_reviews.get(book) {
// ///         Some(review) => println!("{book}: {review}"),
// ///         None => println!("{book} is unreviewed.")
// ///     }
// /// }
// ///
// /// // Look up the value for a key (will panic if the key is not found).
// /// println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
// ///
// /// // Iterate over everything.
// /// for (book, review) in &book_reviews {
// ///     println!("{book}: \"{review}\"");
// /// }
// /// ```
// ///
// /// A `HashMap` with a known list of items can be initialized from an array:
// ///
// /// ```
// /// use std::collections::HashMap;
// ///
// /// let solar_distance = HashMap::from([
// ///     ("Mercury", 0.4),
// ///     ("Venus", 0.7),
// ///     ("Earth", 1.0),
// ///     ("Mars", 1.5),
// /// ]);
// /// ```
// ///
// /// `HashMap` implements an [`Entry` API](#method.entry), which allows
// /// for complex methods of getting, setting, updating and removing keys and
// /// their values:
// ///
// /// ```
// /// use std::collections::HashMap;
// ///
// /// // type inference lets us omit an explicit type signature (which
// /// // would be `HashMap<&str, u8>` in this example).
// /// let mut player_stats = HashMap::new();
// ///
// /// fn random_stat_buff() -> u8 {
// ///     // could actually return some random value here - let's just return
// ///     // some fixed value for now
// ///     42
// /// }
// ///
// /// // insert a key only if it doesn't already exist
// /// player_stats.entry("health").or_insert(100);
// ///
// /// // insert a key using a function that provides a new value only if it
// /// // doesn't already exist
// /// player_stats.entry("defence").or_insert_with(random_stat_buff);
// ///
// /// // update a key, guarding against the key possibly not being set
// /// let stat = player_stats.entry("attack").or_insert(100);
// /// *stat += random_stat_buff();
// ///
// /// // modify an entry before an insert with in-place mutation
// /// player_stats.entry("mana").and_modify(|mana| *mana += 200).or_insert(100);
// /// ```
// ///
// /// The easiest way to use `HashMap` with a custom key type is to derive [`Eq`] and [`Hash`].
// /// We must also derive [`PartialEq`].
// ///
// /// [`RefCell`]: crate::cell::RefCell
// /// [`Cell`]: crate::cell::Cell
// /// [`default`]: Default::default
// /// [`with_hasher`]: Self::with_hasher
// /// [`with_capacity_and_hasher`]: Self::with_capacity_and_hasher
// ///
// /// ```
// /// use std::collections::HashMap;
// ///
// /// #[derive(Hash, Eq, PartialEq, Debug)]
// /// struct Viking {
// ///     name: String,
// ///     country: String,
// /// }
// ///
// /// impl Viking {
// ///     /// Creates a new Viking.
// ///     fn new(name: &str, country: &str) -> Viking {
// ///         Viking { name: name.to_string(), country: country.to_string() }
// ///     }
// /// }
// ///
// /// // Use a HashMap to store the vikings' health points.
// /// let vikings = HashMap::from([
// ///     (Viking::new("Einar", "Norway"), 25),
// ///     (Viking::new("Olaf", "Denmark"), 24),
// ///     (Viking::new("Harald", "Iceland"), 12),
// /// ]);
// ///
// /// // Use derived implementation to print the status of the vikings.
// /// for (viking, health) in &vikings {
// ///     println!("{viking:?} has {health} hp");
// /// }
// /// ```
//
// #[cfg_attr(not(test), rustc_diagnostic_item = "HashMap")]
// #[stable(feature = "rust1", since = "1.0.0")]
// #[rustc_insignificant_dtor]
pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}
//
impl<K, V> HashMap<K, V, RandomState> {
//     /// Creates an empty `HashMap`.
//     ///
//     /// The hash map is initially created with a capacity of 0, so it will not allocate until it
//     /// is first inserted into.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     /// let mut map: HashMap<&str, i32> = HashMap::new();
//     /// ```
//     #[inline]
//     #[must_use]
//     #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new() -> HashMap<K, V, RandomState> {
        Default::default()
    }
//
//     /// Creates an empty `HashMap` with at least the specified capacity.
//     ///
//     /// The hash map will be able to hold at least `capacity` elements without
//     /// reallocating. This method is allowed to allocate for more elements than
//     /// `capacity`. If `capacity` is 0, the hash map will not allocate.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     /// let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
//     /// ```
//     #[inline]
//     #[must_use]
//     #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn with_capacity(capacity: usize) -> HashMap<K, V, RandomState> {
//         HashMap::with_capacity_and_hasher(capacity, Default::default())
//     }
}
//
impl<K, V, S> HashMap<K, V, S> {
//     /// Creates an empty `HashMap` which will use the given hash builder to hash
//     /// keys.
//     ///
//     /// The created map has the default initial capacity.
//     ///
//     /// Warning: `hash_builder` is normally randomly generated, and
//     /// is designed to allow HashMaps to be resistant to attacks that
//     /// cause many collisions and very poor performance. Setting it
//     /// manually using this function can expose a DoS attack vector.
//     ///
//     /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
//     /// the HashMap to be useful, see its documentation for details.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     /// use std::hash::RandomState;
//     ///
//     /// let s = RandomState::new();
//     /// let mut map = HashMap::with_hasher(s);
//     /// map.insert(1, 2);
//     /// ```
    #[inline]
    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap { base: base::HashMap::with_hasher(hash_builder) }
    }
//
//
//     /// An iterator visiting all key-value pairs in arbitrary order.
//     /// The iterator element type is `(&'a K, &'a V)`.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     ///
//     /// let map = HashMap::from([
//     ///     ("a", 1),
//     ///     ("b", 2),
//     ///     ("c", 3),
//     /// ]);
//     ///
//     /// for (key, val) in map.iter() {
//     ///     println!("key: {key} val: {val}");
//     /// }
//     /// ```
//     ///
//     /// # Performance
//     ///
//     /// In the current implementation, iterating over map takes O(capacity) time
//     /// instead of O(len) because it internally visits empty buckets too.
//     #[rustc_lint_query_instability]
//     #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }
//
//
//     /// Returns a reference to the map's [`BuildHasher`].
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     /// use std::hash::RandomState;
//     ///
//     /// let hasher = RandomState::new();
//     /// let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);
//     /// let hasher: &RandomState = map.hasher();
//     /// ```
//     #[inline]
//     #[stable(feature = "hashmap_public_hasher", since = "1.9.0")]
//     pub fn hasher(&self) -> &S {
//         self.base.hasher()
//     }
}
//
impl<K, V, S> HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
{
//
//     /// Inserts a key-value pair into the map.
//     ///
//     /// If the map did not have this key present, [`None`] is returned.
//     ///
//     /// If the map did have this key present, the value is updated, and the old
//     /// value is returned. The key is not updated, though; this matters for
//     /// types that can be `==` without being identical. See the [module-level
//     /// documentation] for more.
//     ///
//     /// [module-level documentation]: crate::collections#insert-and-complex-keys
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     ///
//     /// let mut map = HashMap::new();
//     /// assert_eq!(map.insert(37, "a"), None);
//     /// assert_eq!(map.is_empty(), false);
//     ///
//     /// map.insert(37, "b");
//     /// assert_eq!(map.insert(37, "c"), Some("b"));
//     /// assert_eq!(map[&37], "c");
//     /// ```
//     #[inline]
//     #[stable(feature = "rust1", since = "1.0.0")]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
//
//     /// Tries to insert a key-value pair into the map, and returns
//     /// a mutable reference to the value in the entry.
//     ///
//     /// If the map already had this key present, nothing is updated, and
//     /// an error containing the occupied entry and the value is returned.
//     ///
//     /// # Examples
//     ///
//     /// Basic usage:
//     ///
//     /// ```
//     /// #![feature(map_try_insert)]
//     ///
//     /// use std::collections::HashMap;
//     ///
//     /// let mut map = HashMap::new();
//     /// assert_eq!(map.try_insert(37, "a").unwrap(), &"a");
//     ///
//     /// let err = map.try_insert(37, "b").unwrap_err();
//     /// assert_eq!(err.entry.key(), &37);
//     /// assert_eq!(err.entry.get(), &"a");
//     /// assert_eq!(err.value, "b");
//     /// ```
//     #[unstable(feature = "map_try_insert", issue = "82766")]
//     pub fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, OccupiedError<'_, K, V>> {
//         match self.entry(key) {
//             Occupied(entry) => Err(OccupiedError { entry, value }),
//             Vacant(entry) => Ok(entry.insert(value)),
//         }
//     }
//
//     /// Removes a key from the map, returning the value at the key if the key
//     /// was previously in the map.
//     ///
//     /// The key may be any borrowed form of the map's key type, but
//     /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
//     /// the key type.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     ///
//     /// let mut map = HashMap::new();
//     /// map.insert(1, "a");
//     /// assert_eq!(map.remove(&1), Some("a"));
//     /// assert_eq!(map.remove(&1), None);
//     /// ```
//     #[inline]
//     #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
//         where
//             K: Borrow<Q>,
//             Q: Hash + Eq,
//     {
//         self.base.remove(k)
//     }
//
//     /// Removes a key from the map, returning the stored key and value if the
//     /// key was previously in the map.
//     ///
//     /// The key may be any borrowed form of the map's key type, but
//     /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
//     /// the key type.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     ///
//     /// # fn main() {
//     /// let mut map = HashMap::new();
//     /// map.insert(1, "a");
//     /// assert_eq!(map.remove_entry(&1), Some((1, "a")));
//     /// assert_eq!(map.remove(&1), None);
//     /// # }
//     /// ```
//     #[inline]
//     #[stable(feature = "hash_map_remove_entry", since = "1.27.0")]
//     pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
//         where
//             K: Borrow<Q>,
//             Q: Hash + Eq,
//     {
//         self.base.remove_entry(k)
//     }
}
//
//
// #[stable(feature = "rust1", since = "1.0.0")]
impl<K, V, S> Default for HashMap<K, V, S>
    where
        S: Default,
{
    /// Creates an empty `HashMap<K, V, S>`, with the `Default` value for the hasher.
    #[inline]
    fn default() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}
//
//
// /// An iterator over the entries of a `HashMap`.
// ///
// /// This `struct` is created by the [`iter`] method on [`HashMap`]. See its
// /// documentation for more.
// ///
// /// [`iter`]: HashMap::iter
// ///
// /// # Example
// ///
// /// ```
// /// use std::collections::HashMap;
// ///
// /// let map = HashMap::from([
// ///     ("a", 1),
// /// ]);
// /// let iter = map.iter();
// /// ```
// #[stable(feature = "rust1", since = "1.0.0")]
pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}
//

//
// #[stable(feature = "rust1", since = "1.0.0")]
// impl<K, V, S> IntoIterator for HashMap<K, V, S> {
//     type Item = (K, V);
//     type IntoIter = IntoIter<K, V>;
//
//     /// Creates a consuming iterator, that is, one that moves each key-value
//     /// pair out of the map in arbitrary order. The map cannot be used after
//     /// calling this.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::collections::HashMap;
//     ///
//     /// let map = HashMap::from([
//     ///     ("a", 1),
//     ///     ("b", 2),
//     ///     ("c", 3),
//     /// ]);
//     ///
//     /// // Not possible with .iter()
//     /// let vec: Vec<(&str, i32)> = map.into_iter().collect();
//     /// ```
//     #[inline]
//     #[rustc_lint_query_instability]
//     fn into_iter(self) -> IntoIter<K, V> {
//         IntoIter { base: self.base.into_iter() }
//     }
// }
//
// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

