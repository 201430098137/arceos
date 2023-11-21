use core::time::Duration;
// //! This module exists to isolate [`RandomState`] and [`DefaultHasher`] outside of the
// //! [`collections`] module without actually publicly exporting them, so that parts of that
// //! implementation can more easily be moved to the [`alloc`] crate.
// //!
// //! Although its items are public and contain stability attributes, they can't actually be accessed
// //! outside this crate.
// //!
// //! [`collections`]: crate::collections
// #[allow(deprecated)]
use super::{BuildHasher, Hasher};
use arceos_api::time::AxTimeValue;
// use crate::cell::Cell;
// use crate::fmt;
// use crate::sys;
//
// /// `RandomState` is the default state for [`HashMap`] types.
// ///
// /// A particular instance `RandomState` will create the same instances of
// /// [`Hasher`], but the hashers created by two different `RandomState`
// /// instances are unlikely to produce the same result for the same values.
// ///
// /// [`HashMap`]: crate::collections::HashMap
// ///
// /// # Examples
// ///
// /// ```
// /// use std::collections::HashMap;
// /// use std::hash::RandomState;
// ///
// /// let s = RandomState::new();
// /// let mut map = HashMap::with_hasher(s);
// /// map.insert(1, 2);
// /// ```

use spinlock::SpinNoIrq;
use crate::time;
static PARK_MILLER_LEHMER_SEED: SpinNoIrq<u32> = SpinNoIrq::new(0);
const RAND_MAX: u64 = 2_147_483_647;


#[derive(Clone)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}
//
impl RandomState {
//     /// Constructs a new `RandomState` that is initialized with random keys.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::hash::RandomState;
//     ///
//     /// let s = RandomState::new();
//     /// ```
//     #[inline]
//     #[allow(deprecated)]
//     // rand
//     #[must_use]
//     #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
    pub fn new() -> RandomState {
        // Historically this function did not cache keys from the OS and instead
        // simply always called `rand::thread_rng().gen()` twice. In #31356 it
        // was discovered, however, that because we re-seed the thread-local RNG
        // from the OS periodically that this can cause excessive slowdown when
        // many hash maps are created on a thread. To solve this performance
        // trap we cache the first set of randomly generated keys per-thread.
        //
        // Later in #36481 it was discovered that exposing a deterministic
        // iteration order allows a form of DOS attack. To counter that we
        // increment one of the seeds on every RandomState creation, giving
        // every corresponding HashMap a different iteration order.
        // thread_local!(static KEYS: Cell<(u64, u64)> = {
        //     Cell::new(sys::hashmap_random_keys())
        // });

        // KEYS.with(|keys| {
        //     let (k0, k1) = keys.get();
        //     keys.set((k0.wrapping_add(1), k1));
        //     RandomState { k0, k1 }
        // })
        RandomState { k0:0, k1:0 }
    }
}
//
// #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher()
    }
}
//
// /// The default [`Hasher`] used by [`RandomState`].
// ///
// /// The internal algorithm is not specified, and so it and its hashes should
// /// not be relied upon over releases.
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct DefaultHasher();
//
impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    ///
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    #[inline]
    #[allow(deprecated)]
    #[must_use]
    pub const fn new() -> DefaultHasher {
        DefaultHasher()
    }
}
//
// #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
impl Default for DefaultHasher {
    /// Creates a new `DefaultHasher` using [`new`].
    /// See its documentation for more.
    ///
    /// [`new`]: DefaultHasher::new
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}
//
// #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
impl Hasher for DefaultHasher {
    // The underlying `SipHasher13` doesn't override the other
    // `write_*` methods, so it's ok not to forward them here.

    #[inline]
    fn write(&mut self, msg: &[u8]) {
       // self.0.write(msg)
    }

    #[inline]
    fn write_str(&mut self, s: &str) {
        //self.0.write_str(s);
    }

    #[inline]
    fn finish(&self) -> u64 {
        //self.0.finish()
        return random() as u64;
    }
}
//
// #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
impl Default for RandomState {
    /// Constructs a new `RandomState`.
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
    }
}


pub fn random() -> u128 {
    let mut seed = PARK_MILLER_LEHMER_SEED.lock();
    if *seed == 0 {
        let current_time = arceos_api::time::ax_current_time() as Duration;

        *seed = current_time.as_secs() as u32;
    }
    let mut ret: u128 = 0;
    for _ in 0..4 {
        *seed = ((u64::from(*seed) * 48271) % RAND_MAX) as u32;
        ret = (ret << 32) | (*seed as u128);
    }
    ret
}
//
// #[stable(feature = "std_debug", since = "1.16.0")]
// impl fmt::Debug for RandomState {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("RandomState").finish_non_exhaustive()
//     }
// }
