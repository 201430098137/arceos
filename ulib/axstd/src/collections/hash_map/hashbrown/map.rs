use super::raw::{
    Allocator, Global, RawTable,RawIter, Bucket
};
use super::{Equivalent};
//use crate::{Equivalent, TryReserveError};
// use core::borrow::Borrow;
// use core::fmt::{self, Debug};
use core::hash::{BuildHasher, Hash};
// use core::iter::{FromIterator, FusedIterator};
use core::marker::PhantomData;
use core::mem;
// use core::ops::Index;
//
// /// Default hasher for `HashMap`.
// #[cfg(feature = "ahash")]
// pub type DefaultHashBuilder = core::hash::BuildHasherDefault<ahash::AHasher>;
//
// /// Dummy default hasher for `HashMap`.
#[cfg(not(feature = "ahash"))]
pub enum DefaultHashBuilder {}
//
// /// A hash map implemented with quadratic probing and SIMD lookup.
// ///
// /// The default hashing algorithm is currently [`AHash`], though this is
// /// subject to change at any point in the future. This hash function is very
// /// fast for all types of keys, but this algorithm will typically *not* protect
// /// against attacks such as HashDoS.
// ///
// /// The hashing algorithm can be replaced on a per-`HashMap` basis using the
// /// [`default`], [`with_hasher`], and [`with_capacity_and_hasher`] methods. Many
// /// alternative algorithms are available on crates.io, such as the [`fnv`] crate.
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
// ///
// /// It is a logic error for a key to be modified in such a way that the key's
// /// hash, as determined by the [`Hash`] trait, or its equality, as determined by
// /// the [`Eq`] trait, changes while it is in the map. This is normally only
// /// possible through [`Cell`], [`RefCell`], global state, I/O, or unsafe code.
// ///
// /// It is also a logic error for the [`Hash`] implementation of a key to panic.
// /// This is generally only possible if the trait is implemented manually. If a
// /// panic does occur then the contents of the `HashMap` may become corrupted and
// /// some items may be dropped from the table.
// ///
// /// # Examples
// ///
// /// ```
// /// use hashbrown::HashMap;
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
// ///         Some(review) => println!("{}: {}", book, review),
// ///         None => println!("{} is unreviewed.", book)
// ///     }
// /// }
// ///
// /// // Look up the value for a key (will panic if the key is not found).
// /// println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
// ///
// /// // Iterate over everything.
// /// for (book, review) in &book_reviews {
// ///     println!("{}: \"{}\"", book, review);
// /// }
// /// ```
// ///
// /// `HashMap` also implements an [`Entry API`](#method.entry), which allows
// /// for more complex methods of getting, setting, updating and removing keys and
// /// their values:
// ///
// /// ```
// /// use hashbrown::HashMap;
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
// /// ```
// ///
// /// The easiest way to use `HashMap` with a custom key type is to derive [`Eq`] and [`Hash`].
// /// We must also derive [`PartialEq`].
// ///
// /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
// /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
// /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
// /// [`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
// /// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
// /// [`default`]: #method.default
// /// [`with_hasher`]: #method.with_hasher
// /// [`with_capacity_and_hasher`]: #method.with_capacity_and_hasher
// /// [`fnv`]: https://crates.io/crates/fnv
// /// [`AHash`]: https://crates.io/crates/ahash
// ///
// /// ```
// /// use hashbrown::HashMap;
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
// /// let mut vikings = HashMap::new();
// ///
// /// vikings.insert(Viking::new("Einar", "Norway"), 25);
// /// vikings.insert(Viking::new("Olaf", "Denmark"), 24);
// /// vikings.insert(Viking::new("Harald", "Iceland"), 12);
// ///
// /// // Use derived implementation to print the status of the vikings.
// /// for (viking, health) in &vikings {
// ///     println!("{:?} has {} hp", viking, health);
// /// }
// /// ```
// ///
// /// A `HashMap` with fixed list of elements can be initialized from an array:
// ///
// /// ```
// /// use hashbrown::HashMap;
// ///
// /// let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
// ///     .iter().cloned().collect();
// /// // use the values stored in map
// /// ```
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
//
// impl<K: Clone, V: Clone, S: Clone, A: Allocator + Clone> Clone for HashMap<K, V, S, A> {
//     fn clone(&self) -> Self {
//         HashMap {
//             hash_builder: self.hash_builder.clone(),
//             table: self.table.clone(),
//         }
//     }
//
//     fn clone_from(&mut self, source: &Self) {
//         self.table.clone_from(&source.table);
//
//         // Update hash_builder only if we successfully cloned all elements.
//         self.hash_builder.clone_from(&source.hash_builder);
//     }
// }
//
// /// Ensures that a single closure type across uses of this which, in turn prevents multiple
// /// instances of any functions like RawTable::reserve from being generated
// #[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hasher<Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
where
    Q: Hash,
    S: BuildHasher,
{
    move |val| make_hash::<Q, S>(hash_builder, &val.0)
}
//
// /// Ensures that a single closure type across uses of this which, in turn prevents multiple
// /// instances of any functions like RawTable::reserve from being generated
// #[cfg_attr(feature = "inline-more", inline)]
fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
where
    Q: ?Sized + Equivalent<K>,
{
    move |x| k.equivalent(&x.0)
}
//
// /// Ensures that a single closure type across uses of this which, in turn prevents multiple
// /// instances of any functions like RawTable::reserve from being generated
// #[cfg_attr(feature = "inline-more", inline)]
fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
where
    Q: ?Sized + Equivalent<K>,
{
    move |x| k.equivalent(x)
}
//
// #[cfg(not(feature = "nightly"))]
// #[cfg_attr(feature = "inline-more", inline)]
// pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
// where
//     Q: Hash + ?Sized,
//     S: BuildHasher,
// {
//     use core::hash::Hasher;
//     let mut state = hash_builder.build_hasher();
//     val.hash(&mut state);
//     state.finish()
// }
//
// #[cfg(feature = "nightly")]
// #[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    hash_builder.hash_one(val)
}
//

//
// #[cfg(feature = "ahash")]
// impl<K, V, A: Allocator> HashMap<K, V, DefaultHashBuilder, A> {
//     /// Creates an empty `HashMap` using the given allocator.
//     ///
//     /// The hash map is initially created with a capacity of 0, so it will not allocate until it
//     /// is first inserted into.
//     ///
//     /// # HashDoS resistance
//     ///
//     /// The `hash_builder` normally use a fixed key by default and that does
//     /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
//     /// Users who require HashDoS resistance should explicitly use
//     /// [`ahash::RandomState`] or [`std::collections::hash_map::RandomState`]
//     /// as the hasher when creating a [`HashMap`], for example with
//     /// [`with_hasher_in`](HashMap::with_hasher_in) method.
//     ///
//     /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
//     /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     /// use bumpalo::Bump;
//     ///
//     /// let bump = Bump::new();
//     /// let mut map = HashMap::new_in(&bump);
//     ///
//     /// // The created HashMap holds none elements
//     /// assert_eq!(map.len(), 0);
//     ///
//     /// // The created HashMap also doesn't allocate memory
//     /// assert_eq!(map.capacity(), 0);
//     ///
//     /// // Now we insert element inside created HashMap
//     /// map.insert("One", 1);
//     /// // We can see that the HashMap holds 1 element
//     /// assert_eq!(map.len(), 1);
//     /// // And it also allocates some capacity
//     /// assert!(map.capacity() > 1);
//     /// ```
//     #[cfg_attr(feature = "inline-more", inline)]
//     pub fn new_in(alloc: A) -> Self {
//         Self::with_hasher_in(DefaultHashBuilder::default(), alloc)
//     }
//
//     /// Creates an empty `HashMap` with the specified capacity using the given allocator.
//     ///
//     /// The hash map will be able to hold at least `capacity` elements without
//     /// reallocating. If `capacity` is 0, the hash map will not allocate.
//     ///
//     /// # HashDoS resistance
//     ///
//     /// The `hash_builder` normally use a fixed key by default and that does
//     /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
//     /// Users who require HashDoS resistance should explicitly use
//     /// [`ahash::RandomState`] or [`std::collections::hash_map::RandomState`]
//     /// as the hasher when creating a [`HashMap`], for example with
//     /// [`with_capacity_and_hasher_in`](HashMap::with_capacity_and_hasher_in) method.
//     ///
//     /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
//     /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     /// use bumpalo::Bump;
//     ///
//     /// let bump = Bump::new();
//     /// let mut map = HashMap::with_capacity_in(5, &bump);
//     ///
//     /// // The created HashMap holds none elements
//     /// assert_eq!(map.len(), 0);
//     /// // But it can hold at least 5 elements without reallocating
//     /// let empty_map_capacity = map.capacity();
//     /// assert!(empty_map_capacity >= 5);
//     ///
//     /// // Now we insert some 5 elements inside created HashMap
//     /// map.insert("One",   1);
//     /// map.insert("Two",   2);
//     /// map.insert("Three", 3);
//     /// map.insert("Four",  4);
//     /// map.insert("Five",  5);
//     ///
//     /// // We can see that the HashMap holds 5 elements
//     /// assert_eq!(map.len(), 5);
//     /// // But its capacity isn't changed
//     /// assert_eq!(map.capacity(), empty_map_capacity)
//     /// ```
//     #[cfg_attr(feature = "inline-more", inline)]
//     pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
//         Self::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::default(), alloc)
//     }
// }
//
impl<K, V, S> HashMap<K, V, S> {
//     /// Creates an empty `HashMap` which will use the given hash builder to hash
//     /// keys.
//     ///
//     /// The hash map is initially created with a capacity of 0, so it will not
//     /// allocate until it is first inserted into.
//     ///
//     /// # HashDoS resistance
//     ///
//     /// The `hash_builder` normally use a fixed key by default and that does
//     /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
//     /// Users who require HashDoS resistance should explicitly use
//     /// [`ahash::RandomState`] or [`std::collections::hash_map::RandomState`]
//     /// as the hasher when creating a [`HashMap`].
//     ///
//     /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
//     /// the HashMap to be useful, see its documentation for details.
//     ///
//     /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
//     /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
//     /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     /// use hashbrown::hash_map::DefaultHashBuilder;
//     ///
//     /// let s = DefaultHashBuilder::default();
//     /// let mut map = HashMap::with_hasher(s);
//     /// assert_eq!(map.len(), 0);
//     /// assert_eq!(map.capacity(), 0);
//     ///
//     /// map.insert(1, 2);
//     /// ```
//     #[cfg_attr(feature = "inline-more", inline)]
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: RawTable::new(),
        }
    }
//
//     /// Creates an empty `HashMap` with the specified capacity, using `hash_builder`
//     /// to hash the keys.
//     ///
//     /// The hash map will be able to hold at least `capacity` elements without
//     /// reallocating. If `capacity` is 0, the hash map will not allocate.
//     ///
//     /// # HashDoS resistance
//     ///
//     /// The `hash_builder` normally use a fixed key by default and that does
//     /// not allow the `HashMap` to be protected against attacks such as [`HashDoS`].
//     /// Users who require HashDoS resistance should explicitly use
//     /// [`ahash::RandomState`] or [`std::collections::hash_map::RandomState`]
//     /// as the hasher when creating a [`HashMap`].
//     ///
//     /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
//     /// the HashMap to be useful, see its documentation for details.
//     ///
//     /// [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack
//     /// [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html
//     /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     /// use hashbrown::hash_map::DefaultHashBuilder;
//     ///
//     /// let s = DefaultHashBuilder::default();
//     /// let mut map = HashMap::with_capacity_and_hasher(10, s);
//     /// assert_eq!(map.len(), 0);
//     /// assert!(map.capacity() >= 10);
//     ///
//     /// map.insert(1, 2);
//     /// ```
//     #[cfg_attr(feature = "inline-more", inline)]
//     pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
//         Self {
//             hash_builder,
//             table: RawTable::with_capacity(capacity),
//         }
//     }
}
//
impl<K, V, S, A: Allocator> HashMap<K, V, S, A> {

//
//     /// An iterator visiting all key-value pairs in arbitrary order.
//     /// The iterator element type is `(&'a K, &'a V)`.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     ///
//     /// let mut map = HashMap::new();
//     /// map.insert("a", 1);
//     /// map.insert("b", 2);
//     /// map.insert("c", 3);
//     /// assert_eq!(map.len(), 3);
//     /// let mut vec: Vec<(&str, i32)> = Vec::new();
//     ///
//     /// for (key, val) in map.iter() {
//     ///     println!("key: {} val: {}", key, val);
//     ///     vec.push((*key, *val));
//     /// }
//     ///
//     /// // The `Iter` iterator produces items in arbitrary order, so the
//     /// // items must be sorted to test them against a sorted array.
//     /// vec.sort_unstable();
//     /// assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);
//     ///
//     /// assert_eq!(map.len(), 3);
//     /// ```
//     #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> Iter<'_, K, V> {
        // Here we tie the lifetime of self to the iter.
        unsafe {
            Iter {
                inner: self.table.iter(),
                marker: PhantomData,
            }
        }
    }

//
//     /// Clears the map, removing all key-value pairs. Keeps the allocated memory
//     /// for reuse.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     ///
//     /// let mut a = HashMap::new();
//     /// a.insert(1, "a");
//     /// let capacity_before_clear = a.capacity();
//     ///
//     /// a.clear();
//     ///
//     /// // Map is empty.
//     /// assert!(a.is_empty());
//     /// // But map capacity is equal to old one.
//     /// assert_eq!(a.capacity(), capacity_before_clear);
//     /// ```
//     #[cfg_attr(feature = "inline-more", inline)]
//     pub fn clear(&mut self) {
//         self.table.clear();
//     }
//
//     /// Creates a consuming iterator visiting all the keys in arbitrary order.
//     /// The map cannot be used after calling this.
//     /// The iterator element type is `K`.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     ///
//     /// let mut map = HashMap::new();
//     /// map.insert("a", 1);
//     /// map.insert("b", 2);
//     /// map.insert("c", 3);
//     ///
//     /// let mut vec: Vec<&str> = map.into_keys().collect();
//     ///
//     /// // The `IntoKeys` iterator produces keys in arbitrary order, so the
//     /// // keys must be sorted to test them against a sorted array.
//     /// vec.sort_unstable();
//     /// assert_eq!(vec, ["a", "b", "c"]);
//     /// ```
//     #[inline]
//     pub fn into_keys(self) -> IntoKeys<K, V, A> {
//         IntoKeys {
//             inner: self.into_iter(),
//         }
//     }
//
//     /// Creates a consuming iterator visiting all the values in arbitrary order.
//     /// The map cannot be used after calling this.
//     /// The iterator element type is `V`.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     ///
//     /// let mut map = HashMap::new();
//     /// map.insert("a", 1);
//     /// map.insert("b", 2);
//     /// map.insert("c", 3);
//     ///
//     /// let mut vec: Vec<i32> = map.into_values().collect();
//     ///
//     /// // The `IntoValues` iterator produces values in arbitrary order, so
//     /// // the values must be sorted to test them against a sorted array.
//     /// vec.sort_unstable();
//     /// assert_eq!(vec, [1, 2, 3]);
//     /// ```
//     #[inline]
//     pub fn into_values(self) -> IntoValues<K, V, A> {
//         IntoValues {
//             inner: self.into_iter(),
//         }
//     }
}
//
impl<K, V, S, A> HashMap<K, V, S, A>
where
    K: Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{

//
//     /// Inserts a key-value pair into the map.
//     ///
//     /// If the map did not have this key present, [`None`] is returned.
//     ///
//     /// If the map did have this key present, the value is updated, and the old
//     /// value is returned. The key is not updated, though; this matters for
//     /// types that can be `==` without being identical. See the [`std::collections`]
//     /// [module-level documentation] for more.
//     ///
//     /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
//     /// [`std::collections`]: https://doc.rust-lang.org/std/collections/index.html
//     /// [module-level documentation]: https://doc.rust-lang.org/std/collections/index.html#insert-and-complex-keys
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hashbrown::HashMap;
//     ///
//     /// let mut map = HashMap::new();
//     /// assert_eq!(map.insert(37, "a"), None);
//     /// assert_eq!(map.is_empty(), false);
//     ///
//     /// map.insert(37, "b");
//     /// assert_eq!(map.insert(37, "c"), Some("b"));
//     /// assert_eq!(map[&37], "c");
//     /// ```
//     #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = make_hash::<K, S>(&self.hash_builder, &k);
        let hasher = make_hasher::<_, V, S>(&self.hash_builder);
        match self
            .table
            .find_or_find_insert_slot(hash, equivalent_key(&k), hasher)
        {
            Ok(bucket) => Some(mem::replace(unsafe { &mut bucket.as_mut().1 }, v)),
            Err(slot) => {
                unsafe {
                    self.table.insert_in_slot(hash, slot, (k, v));
                }
                None
            }
        }
    }

}

//
// /// An iterator over the entries of a `HashMap` in arbitrary order.
// /// The iterator element type is `(&'a K, &'a V)`.
// ///
// /// This `struct` is created by the [`iter`] method on [`HashMap`]. See its
// /// documentation for more.
// ///
// /// [`iter`]: struct.HashMap.html#method.iter
// /// [`HashMap`]: struct.HashMap.html
// ///
// /// # Examples
// ///
// /// ```
// /// use hashbrown::HashMap;
// ///
// /// let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();
// ///
// /// let mut iter = map.iter();
// /// let mut vec = vec![iter.next(), iter.next(), iter.next()];
// ///
// /// // The `Iter` iterator produces items in arbitrary order, so the
// /// // items must be sorted to test them against a sorted array.
// /// vec.sort_unstable();
// /// assert_eq!(vec, [Some((&1, &"a")), Some((&2, &"b")), Some((&3, &"c"))]);
// ///
// /// // It is fused iterator
// /// assert_eq!(iter.next(), None);
// /// assert_eq!(iter.next(), None);
// /// ```
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}


impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some(x) => unsafe {
                let r = x.as_ref();
                Some((&r.0, &r.1))
            },
            None => None,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
