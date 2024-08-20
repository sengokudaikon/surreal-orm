use std::{
    mem, ops::{Deref, DerefMut}, option::Iter, pin::Pin
};

use serde::Serialize;

/// Represents a value that may or may not be present
#[derive(Default, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Maybe<T> {
    /// When the value is present
    Some(T),
    /// When the value is absent
    #[default]
    None,
}

// impl<T> Deref for Maybe<T> {
//     type Target = Option<T>;
//
//     fn deref(&self) -> &Self::Target {
//         let x = match self {
//             Maybe::Maybe::Some(v) => Maybe::Some(v.clone()),
//             Maybe::None => None,
//         };
//         &x
//     }
// }
// impl<T> DerefMut for Maybe<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut match self {
//             Maybe::Maybe::Some(v) => Maybe::Some(v),
//             Maybe::None => None,
//         }
//     }
// }

// impl<T> Deref for Maybe<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }

impl<T> From<Maybe<T>> for Option<T> {
    fn from(value: Maybe<T>) -> Self {
        match value {
            Maybe::Some(v) => Some(v),
            Maybe::None => None,
        }
    }
}

// impl<T> From<Maybe<&T>> for Option<&T> {
//     fn from(value: Maybe<&T>) -> Self {
//         match value {
//             Maybe::Maybe::Some(v) => Maybe::Some(v),
//             Maybe::None => None,
//         }
//     }
// }

type M = Option<u32>;

impl<T> Maybe<T> {
    /// Checks if the value is present
    pub const fn is_some(&self) -> bool {
        matches!(self, Maybe::Some(_))
    }

    /// Checks if the value is absent
    pub const fn is_none(&self) -> bool {
        matches!(self, Maybe::None)
    }

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the option is a [`Maybe::Some`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Option<u32> = Maybe::Some(2);
    /// assert_eq!(x.is_some(), true);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.is_some(), false);
    /// ```
    // #[must_use = "if you intended to assert that this has a value, consider `.unwrap()` instead"]
    // #[inline]
    // pub const fn is_some(&self) -> bool {
    //     matches!(*self, Maybe::Some(_))
    // }

    /// Returns `true` if the option is a [`Maybe::Some`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Option<u32> = Maybe::Some(2);
    /// assert_eq!(x.is_some_and(|x| x > 1), true);
    ///
    /// let x: Option<u32> = Maybe::Some(0);
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Maybe::None => false,
            Maybe::Some(x) => f(x),
        }
    }

    /// Returns `true` if the option is a [`None`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Option<u32> = Maybe::Some(2);
    /// assert_eq!(x.is_none(), false);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.is_none(), true);
    /// ```
    // #[must_use = "if you intended to assert that this doesn't have a value, consider \
    //               wrapping this in an `assert!()` instead"]
    // #[inline]
    // pub const fn is_none(&self) -> bool {
    //     !self.is_some()
    // }

    /////////////////////////////////////////////////////////////////////////
    // Adapter for working with references
    /////////////////////////////////////////////////////////////////////////

    /// Converts from `&Option<T>` to `Option<&T>`.
    ///
    /// # Examples
    ///
    /// Calculates the length of an <code>Option<[String]></code> as an <code>Option<[usize]></code>
    /// without moving the [`String`]. The [`map`] method takes the `self` argument by value,
    /// consuming the original, so this technique uses `as_ref` to first take an `Option` to a
    /// reference to the value inside the original.
    ///
    /// [`map`]: Option::map
    /// [String]: ../../std/string/struct.String.html "String"
    /// [`String`]: ../../std/string/struct.String.html "String"
    ///
    /// ```
    /// let text: Option<String> = Maybe::Some("Hello, world!".to_string());
    /// // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `text` on the stack.
    /// let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    /// println!("still can print text: {text:?}");
    /// ```
    #[inline]
    pub const fn as_ref(&self) -> Maybe<&T> {
        match *self {
            Maybe::Some(ref x) => Maybe::Some(x),
            Maybe::None => Maybe::None,
        }
    }

    /// Converts from `&mut Option<T>` to `Option<&mut T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut x = Maybe::Some(2);
    /// match x.as_mut() {
    ///     Maybe::Some(v) => *v = 42,
    ///     None => {},
    /// }
    /// assert_eq!(x, Maybe::Some(42));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Maybe<&mut T> {
        match *self {
            Maybe::Some(ref mut x) => Maybe::Some(x),
            Maybe::None => Maybe::None,
        }
    }

    /// Converts from <code>[Pin]<[&]Option\<T>></code> to <code>Option<[Pin]<[&]T>></code>.
    ///
    /// [&]: reference "shared reference"
    // #[inline]
    // #[must_use]
    // pub const fn as_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>> {
    //     let x:Option<T>  =  self.
    //     match Pin::get_ref(self).as_ref() {
    //         // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
    //         // which is pinned.
    //         Maybe::Some(x) => unsafe { Maybe::Some(Pin::new_unchecked(x)) },
    //         None => None,
    //     }
    // }


    /// Returns a slice of the contained value, if any. If this is `None`, an
    /// empty slice is returned. This can be useful to have a single type of
    /// iterator over an `Option` or slice.
    ///
    /// Note: Should you have an `Option<&T>` and wish to get a slice of `T`,
    /// you can unpack it via `opt.map_or(&[], std::slice::from_ref)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     [Maybe::Some(1234).as_slice(), None.as_slice()],
    ///     [&[1234][..], &[][..]],
    /// );
    /// ```
    ///
    /// The inverse of this function is (discounting
    /// borrowing) [`[_]::first`](slice::first):
    ///
    /// ```rust
    /// for i in [Maybe::Some(1234_u16), None] {
    ///     assert_eq!(i.as_ref(), i.as_slice().first());
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: When the `Option` is `Maybe::Some`, we're using the actual pointer
        // to the payload, with a length of 1, so this is equivalent to
        // `slice::from_ref`, and thus is safe.
        // When the `Option` is `None`, the length used is 0, so to be safe it
        // just needs to be aligned, which it is because `&self` is aligned and
        // the offset used is a multiple of alignment.
        //
        // In the new version, the intrinsic always returns a pointer to an
        // in-bounds and correctly aligned position for a `T` (even if in the
        // `None` case it's just padding).
        //     unsafe {
        //         slice::from_raw_parts(
        //             (self as *const Self).byte_add(core::mem::offset_of!(Self, Maybe::Some.0)).cast(),
        //             usize::from(self.is_some()),
        //         )
        //     }
        todo!()
    }


    /////////////////////////////////////////////////////////////////////////
    // Getting to contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns the contained [`Maybe::Some`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`None`] with a custom panic message provided by
    /// `msg`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some("value");
    /// assert_eq!(x.expect("fruits are healthy"), "value");
    /// ```
    ///
    /// ```should_panic
    /// let x: Option<&str> = None;
    /// x.expect("fruits are healthy"); // panics with `fruits are healthy`
    /// ```
    ///
    /// # Recommended Message Style
    ///
    /// We recommend that `expect` messages are used to describe the reason you
    /// _expect_ the `Option` should be `Maybe::Some`.
    ///
    /// ```should_panic
    /// # let slice: &[u8] = &[];
    /// let item = slice.get(0)
    ///     .expect("slice should not be empty");
    /// ```
    ///
    /// **Hint**: If you're having trouble remembering how to phrase expect
    /// error messages remember to focus on the word "should" as in "env
    /// variable should be set by blah" or "the given binary should be available
    /// and executable by the current user".
    ///
    /// For more detail on expect message styles and the reasoning behind our
    /// recommendation please refer to the section on ["Common Message
    /// Styles"](../../std/error/index.html#common-message-styles) in the [`std::error`](../../std/error/index.html) module docs.
    #[inline]
    #[track_caller]
    // pub const fn expect(self, msg: &str) -> T {
    pub fn expect(self, msg: &str) -> T {
        match self {
            Maybe::Some(val) => val,
            Maybe::None => panic!("{}", msg),
        }
    }

    /// Returns the contained [`Maybe::Some`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`None`]
    /// case explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: Option::unwrap_or
    /// [`unwrap_or_else`]: Option::unwrap_or_else
    /// [`unwrap_or_default`]: Option::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some("air");
    /// assert_eq!(x.unwrap(), "air");
    /// ```
    ///
    /// ```should_panic
    /// let x: Option<&str> = None;
    /// assert_eq!(x.unwrap(), "air"); // fails
    /// ```
    #[inline(always)]
    #[track_caller]
    pub const fn unwrap(self) -> T {
        match self {
            Maybe::Some(val) => val,
            Maybe::None => panic!(),
        }
    }

    /// Returns the contained [`Maybe::Some`] value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or_else`]: Option::unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Maybe::Some("car").unwrap_or("bike"), "car");
    /// assert_eq!(None.unwrap_or("bike"), "bike");
    /// ```
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Maybe::Some(x) => x,
            Maybe::None => default,
        }
    }

    /// Returns the contained [`Maybe::Some`] value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// let k = 10;
    /// assert_eq!(Maybe::Some(4).unwrap_or_else(|| 2 * k), 4);
    /// assert_eq!(Maybe::None.unwrap_or_else(|| 2 * k), 20);
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Maybe::Some(x) => x,
            Maybe::None => f(),
        }
    }

    /// Returns the contained [`Maybe::Some`] value or a default.
    ///
    /// Consumes the `self` argument then, if [`Maybe::Some`], returns the contained
    /// value, otherwise if [`Maybe::None`], returns the [default value] for that
    /// type.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Option<u32> = Maybe::None;
    /// let y: Option<u32> = Maybe::Some(12);
    ///
    /// assert_eq!(x.unwrap_or_default(), 0);
    /// assert_eq!(y.unwrap_or_default(), 12);
    /// ```
    ///
    /// [default value]: Default::default
    /// [`parse`]: str::parse
    /// [`FromStr`]: crate::str::FromStr
    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Maybe::Some(x) => x,
            Maybe::None => T::default(),
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////

    /// Maps an `Option<T>` to `Option<U>` by applying a function to a contained value (if `Maybe::Some`) or returns `Maybe::None` (if `Maybe::None`).
    ///
    /// # Examples
    ///
    /// Calculates the length of an <code>Option<[String]></code> as an
    /// <code>Option<[usize]></code>, consuming the original:
    ///
    /// [String]: ../../std/string/struct.String.html "String"
    /// ```
    /// let maybe_some_string = Maybe::Some(String::from("Hello, World!"));
    /// // `Option::map` takes self *by value*, consuming `maybe_some_string`
    /// let maybe_some_len = maybe_some_string.map(|s| s.len());
    /// assert_eq!(maybe_some_len, Maybe::Some(13));
    ///
    /// let x: Option<&str> = Maybe::None;
    /// assert_eq!(x.map(|s| s.len()), Maybe::None);
    /// ```
    #[inline]
    pub fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Maybe::Some(x) => Maybe::Some(f(x)),
            Maybe::None => Maybe::None,
        }
    }

    /// Calls a function with a reference to the contained value if [`Maybe::Some`].
    ///
    /// Returns the original option.
    ///
    /// # Examples
    ///
    /// ```
    /// let list = vec![1, 2, 3];
    ///
    /// // prints "got: 2"
    /// let x = list
    ///     .get(1)
    ///     .inspect(|x| println!("got: {x}"))
    ///     .expect("list should be long enough");
    ///
    /// // prints nothing
    /// list.get(5).inspect(|x| println!("got: {x}"));
    /// ```
    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Maybe::Some(ref x) = self {
            f(x);
        }

        self
    }

    /// Returns the provided default result (if none),
    /// or applies a function to the contained value (if any).
    ///
    /// Arguments passed to `map_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`map_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`map_or_else`]: Option::map_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some("foo");
    /// assert_eq!(x.map_or(42, |v| v.len()), 3);
    ///
    /// let x: Option<&str> = Maybe::None;
    /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    /// ```
    #[inline]
    #[must_use = "if you don't need the returned value, use `if let` instead"]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Maybe::Some(t) => f(t),
            Maybe::None => default,
        }
    }

    /// Computes a default function result (if none), or
    /// applies a different function to the contained value (if any).
    ///
    /// # Basic examples
    ///
    /// ```
    /// let k = 21;
    ///
    /// let x = Maybe::Some("foo");
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
    ///
    /// let x: Option<&str> = Maybe::None;
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
    /// ```
    ///
    /// # Handling a Result-based fallback
    ///
    /// A somewhat common occurrence when dealing with optional values
    /// in combination with [`Result<T, E>`] is the case where one wants to invoke
    /// a fallible fallback if the option is not present.  This example
    /// parses a command line argument (if present), or the contents of a file to
    /// an integer.  However, unlike accessing the command line argument, reading
    /// the file is fallible, so it must be wrapped with `Ok`.
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let v: u64 = std::env::args()
    ///    .nth(1)
    ///    .map_or_else(|| std::fs::read_to_string("/etc/someconfig.conf"), Ok)?
    ///    .parse()?;
    /// #   Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        match self {
            Maybe::Some(t) => f(t),
            Maybe::None => default(),
        }
    }

    /// Transforms the `Option<T>` into a [`Result<T, E>`], mapping [`Maybe::Some(v)`] to
    /// [`Ok(v)`] and [`Maybe::None`] to [`Err(err)`].
    ///
    /// Arguments passed to `ok_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`ok_or_else`], which is
    /// lazily evaluated.
    ///
    /// [`Ok(v)`]: Ok
    /// [`Err(err)`]: Err
    /// [`Maybe::Some(v)`]: Maybe::Some
    /// [`ok_or_else`]: Option::ok_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some("foo");
    /// assert_eq!(x.ok_or(0), Ok("foo"));
    ///
    /// let x: Option<&str> = Maybe::None;
    /// assert_eq!(x.ok_or(0), Err(0));
    /// ```
    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Maybe::Some(v) => Ok(v),
            Maybe::None => Err(err),
        }
    }

    /// Transforms the `Option<T>` into a [`Result<T, E>`], mapping [`Maybe::Some(v)`] to
    /// [`Ok(v)`] and [`Maybe::None`] to [`Err(err())`].
    ///
    /// [`Ok(v)`]: Ok
    /// [`Err(err())`]: Err
    /// [`Maybe::Some(v)`]: Maybe::Some
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some("foo");
    /// assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
    ///
    /// let x: Option<&str> = Maybe::None;
    /// assert_eq!(x.ok_or_else(|| 0), Err(0));
    /// ```
    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Maybe::Some(v) => Ok(v),
            Maybe::None => Err(err()),
        }
    }

    /// Converts from `Option<T>` (or `&Option<T>`) to `Option<&T::Target>`.
    ///
    /// Leaves the original Option in-place, creating a new one with a reference
    /// to the original one, additionally coercing the contents via [`Deref`].
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Option<String> = Maybe::Some("hey".to_owned());
    /// assert_eq!(x.as_deref(), Maybe::Some("hey"));
    ///
    /// let x: Option<String> = Maybe::None;
    /// assert_eq!(x.as_deref(), Maybe::None);
    /// ```
    #[inline]
    pub fn as_deref(&self) -> Maybe<&T::Target>
    where
        T: Deref,
    {
        match self.as_ref() {
            Maybe::Some(t) => Maybe::Some(t.deref()),
            Maybe::None => Maybe::None,
        }
    }

    /// Converts from `Option<T>` (or `&mut Option<T>`) to `Option<&mut T::Target>`.
    ///
    /// Leaves the original `Option` in-place, creating a new one containing a mutable reference to
    /// the inner type's [`Deref::Target`] type.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut x: Option<String> = Maybe::Some("hey".to_owned());
    /// assert_eq!(x.as_deref_mut().map(|x| {
    ///     x.make_ascii_uppercase();
    ///     x
    /// }), Maybe::Some("HEY".to_owned().as_mut_str()));
    /// ```
    #[inline]
    pub fn as_deref_mut(&mut self) -> Maybe<&mut T::Target>
    where
        T: DerefMut,
    {
        match self.as_mut() {
            Maybe::Some(t) => Maybe::Some(t.deref_mut()),
            Maybe::None => Maybe::None,
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Boolean operations on the values, eager and lazy
    /////////////////////////////////////////////////////////////////////////

    /// Returns [`Maybe::None`] if the option is [`Maybe::None`], otherwise returns `optb`.
    ///
    /// Arguments passed to `and` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`and_then`], which is
    /// lazily evaluated.
    ///
    /// [`and_then`]: Option::and_then
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some(2);
    /// let y: Option<&str> = Maybe::None;
    /// assert_eq!(x.and(y), Maybe::None);
    ///
    /// let x: Option<u32> = Maybe::None;
    /// let y = Maybe::Some("foo");
    /// assert_eq!(x.and(y), Maybe::None);
    ///
    /// let x = Maybe::Some(2);
    /// let y = Maybe::Some("foo");
    /// assert_eq!(x.and(y), Maybe::Some("foo"));
    ///
    /// let x: Option<u32> = Maybe::None;
    /// let y: Option<&str> = Maybe::None;
    /// assert_eq!(x.and(y), None);
    /// ```
    #[inline]
    pub fn and<U>(self, optb: Maybe<U>) -> Maybe<U> {
        match self {
            Maybe::Some(_) => optb,
            Maybe::None => Maybe::None,
        }
    }

    /// Returns [`None`] if the option is [`None`], otherwise calls `f` with the
    /// wrapped value and returns the result.
    ///
    /// Maybe::Some languages call this operation flatmap.
    ///
    /// # Examples
    ///
    /// ```
    /// fn sq_then_to_string(x: u32) -> Option<String> {
    ///     x.checked_mul(x).map(|sq| sq.to_string())
    /// }
    ///
    /// assert_eq!(Maybe::Some(2).and_then(sq_then_to_string), Maybe::Some(4.to_string()));
    /// assert_eq!(Maybe::Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
    /// assert_eq!(None.and_then(sq_then_to_string), None);
    /// ```
    ///
    /// Often used to chain fallible operations that may return [`None`].
    ///
    /// ```
    /// let arr_2d = [["A0", "A1"], ["B0", "B1"]];
    ///
    /// let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
    /// assert_eq!(item_0_1, Maybe::Some(&"A1"));
    ///
    /// let item_2_0 = arr_2d.get(2).and_then(|row| row.get(0));
    /// assert_eq!(item_2_0, None);
    /// ```
    #[doc(alias = "flatmap")]
    #[inline]
    pub fn and_then<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> Maybe<U>,
    {
        match self {
            Maybe::Some(x) => f(x),
            Maybe::None => Maybe::None,
        }
    }

    /// Returns [`None`] if the option is [`None`], otherwise calls `predicate`
    /// with the wrapped value and returns:
    ///
    /// - [`Maybe::Some(t)`] if `predicate` returns `true` (where `t` is the wrapped
    ///   value), and
    /// - [`None`] if `predicate` returns `false`.
    ///
    /// This function works similar to [`Iterator::filter()`]. You can imagine
    /// the `Option<T>` being an iterator over one or zero elements. `filter()`
    /// lets you decide which elements to keep.
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn is_even(n: &i32) -> bool {
    ///     n % 2 == 0
    /// }
    ///
    /// assert_eq!(None.filter(is_even), None);
    /// assert_eq!(Maybe::Some(3).filter(is_even), None);
    /// assert_eq!(Maybe::Some(4).filter(is_even), Maybe::Some(4));
    /// ```
    ///
    /// [`Maybe::Some(t)`]: Maybe::Some
    #[inline]
    pub fn filter<P>(self, predicate: P) -> Self
    where
        P: FnOnce(&T) -> bool,
    {
        if let Maybe::Some(x) = self {
            if predicate(&x) {
                return Maybe::Some(x);
            }
        }
        Maybe::None
    }

    /// Returns the option if it contains a value, otherwise returns `optb`.
    ///
    /// Arguments passed to `or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`or_else`], which is
    /// lazily evaluated.
    ///
    /// [`or_else`]: Option::or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some(2);
    /// let y = None;
    /// assert_eq!(x.or(y), Maybe::Some(2));
    ///
    /// let x = None;
    /// let y = Maybe::Some(100);
    /// assert_eq!(x.or(y), Maybe::Some(100));
    ///
    /// let x = Maybe::Some(2);
    /// let y = Maybe::Some(100);
    /// assert_eq!(x.or(y), Maybe::Some(2));
    ///
    /// let x: Option<u32> = None;
    /// let y = None;
    /// assert_eq!(x.or(y), None);
    /// ```
    #[inline]
    pub fn or(self, optb: Maybe<T>) -> Maybe<T> {
        match self {
            x @ Maybe::Some(_) => x,
            Maybe::None => optb,
        }
    }

    /// Returns the option if it contains a value, otherwise calls `f` and
    /// returns the result.
    ///
    /// # Examples
    ///
    /// ```
    /// fn nobody() -> Option<&'static str> { None }
    /// fn vikings() -> Option<&'static str> { Maybe::Some("vikings") }
    ///
    /// assert_eq!(Maybe::Some("barbarians").or_else(vikings), Maybe::Some("barbarians"));
    /// assert_eq!(None.or_else(vikings), Maybe::Some("vikings"));
    /// assert_eq!(None.or_else(nobody), None);
    /// ```
    #[inline]
    pub fn or_else<F>(self, f: F) -> Maybe<T>
    where
        F: FnOnce() -> Maybe<T>,
    {
        match self {
            x @ Maybe::Some(_) => x,
            Maybe::None => f(),
        }
    }

    /// Returns [`Maybe::Some`] if exactly one of `self`, `optb` is [`Maybe::Some`], otherwise returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some(2);
    /// let y: Option<u32> = None;
    /// assert_eq!(x.xor(y), Maybe::Some(2));
    ///
    /// let x: Option<u32> = None;
    /// let y = Maybe::Some(2);
    /// assert_eq!(x.xor(y), Maybe::Some(2));
    ///
    /// let x = Maybe::Some(2);
    /// let y = Maybe::Some(2);
    /// assert_eq!(x.xor(y), None);
    ///
    /// let x: Option<u32> = None;
    /// let y: Option<u32> = None;
    /// assert_eq!(x.xor(y), None);
    /// ```
    #[inline]
    pub fn xor(self, optb: Maybe<T>) -> Maybe<T> {
        match (self, optb) {
            (a @ Maybe::Some(_), Maybe::None) => a,
            (Maybe::None, b @ Maybe::Some(_)) => b,
            _ => Maybe::None,
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Entry-like operations to insert a value and return a reference
    /////////////////////////////////////////////////////////////////////////



    /////////////////////////////////////////////////////////////////////////
    // Misc
    /////////////////////////////////////////////////////////////////////////

    /// Takes the value out of the option, leaving a [`None`] in its place.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut x = Maybe::Some(2);
    /// let y = x.take();
    /// assert_eq!(x, None);
    /// assert_eq!(y, Maybe::Some(2));
    ///
    /// let mut x: Option<u32> = None;
    /// let y = x.take();
    /// assert_eq!(x, None);
    /// assert_eq!(y, None);
    /// ```
    #[inline]
    pub fn take(&mut self) -> Maybe<T> {
        // FIXME replace `mem::replace` by `mem::take` when the latter is const ready
        mem::replace(self, Maybe::None)
    }

    /// Takes the value out of the option, but only if the predicate evaluates to
    /// `true` on a mutable reference to the value.
    ///
    /// In other words, replaces `self` with `None` if the predicate returns `true`.
    /// This method operates similar to [`Option::take`] but conditional.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut x = Maybe::Some(42);
    ///
    /// let prev = x.take_if(|v| if *v == 42 {
    ///     *v += 1;
    ///     false
    /// } else {
    ///     false
    /// });
    /// assert_eq!(x, Maybe::Some(43));
    /// assert_eq!(prev, None);
    ///
    /// let prev = x.take_if(|v| *v == 43);
    /// assert_eq!(x, None);
    /// assert_eq!(prev, Maybe::Some(43));
    /// ```
    #[inline]
    pub fn take_if<P>(&mut self, predicate: P) -> Maybe<T>
    where
        P: FnOnce(&mut T) -> bool,
    {
        if self.as_mut().map_or(false, predicate) {
            self.take()
        } else {
            Maybe::None
        }
    }

    /// Replaces the actual value in the option by the value given in parameter,
    /// returning the old value if present,
    /// leaving a [`Maybe::Some`] in its place without deinitializing either one.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut x = Maybe::Some(2);
    /// let old = x.replace(5);
    /// assert_eq!(x, Maybe::Some(5));
    /// assert_eq!(old, Maybe::Some(2));
    ///
    /// let mut x = None;
    /// let old = x.replace(3);
    /// assert_eq!(x, Maybe::Some(3));
    /// assert_eq!(old, None);
    /// ```
    #[inline]
    pub fn replace(&mut self, value: T) -> Maybe<T> {
        mem::replace(self, Maybe::Some(value))
    }

    /// Zips `self` with another `Option`.
    ///
    /// If `self` is `Maybe::Some(s)` and `other` is `Maybe::Some(o)`, this method returns `Maybe::Some((s, o))`.
    /// Otherwise, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Maybe::Some(1);
    /// let y = Maybe::Some("hi");
    /// let z = None::<u8>;
    ///
    /// assert_eq!(x.zip(y), Maybe::Some((1, "hi")));
    /// assert_eq!(x.zip(z), None);
    /// ```
    pub fn zip<U>(self, other: Maybe<U>) -> Maybe<(T, U)> {
        match (self, other) {
            (Maybe::Some(a), Maybe::Some(b)) => Maybe::Some((a, b)),
            _ => Maybe::None,
        }
    }

    /// Zips `self` and another `Option` with function `f`.
    ///
    /// If `self` is `Maybe::Some(s)` and `other` is `Maybe::Some(o)`, this method returns `Maybe::Some(f(s, o))`.
    /// Otherwise, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(option_zip)]
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Point {
    ///     x: f64,
    ///     y: f64,
    /// }
    ///
    /// impl Point {
    ///     fn new(x: f64, y: f64) -> Self {
    ///         Self { x, y }
    ///     }
    /// }
    ///
    /// let x = Maybe::Some(17.5);
    /// let y = Maybe::Some(42.7);
    ///
    /// assert_eq!(x.zip_with(y, Point::new), Maybe::Some(Point { x: 17.5, y: 42.7 }));
    /// assert_eq!(x.zip_with(None, Point::new), None);
    /// ```
    pub fn zip_with<U, F, R>(self, other: Maybe<U>, f: F) -> Maybe<R>
    where
        F: FnOnce(T, U) -> R,
    {
        match (self, other) {
            (Maybe::Some(a), Maybe::Some(b)) => Maybe::Some(f(a, b)),
            _ => Maybe::None,
        }
    }
}
