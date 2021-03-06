// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Usage: regex!(pattern)
//
// Builds a ::Regex from a borrowed string.
//
// Due to macro scoping rules, this definition only applies for the modules
// defined below. Effectively, it allows us to use the same tests for both
// native and dynamic regexes.
macro_rules! regex {
    ($re:expr) => {
        ::regex::Regex::new(&$re.to_owned()).unwrap()
    };
}

// Usage: text!(haystack)
//
// Builds a ::Text from an owned string.
//
// This macro is called on every input searched in every benchmark. It is
// called exactly once per benchmark and its time is not included in the
// benchmark timing.
//
// The text given to the macro is always a String, which is guaranteed to be
// valid UTF-8.
//
// The return type should be an owned value that can deref to whatever the
// regex accepts in its `is_match` and `find_iter` methods.

macro_rules! text {
    ($text:expr) => {
        $text
    };
}

// Macros for writing benchmarks easily. We provide macros for benchmarking
// matches, non-matches and for finding all successive non-overlapping matches
// in a string (including a check that the count is correct).

// USAGE: bench_match!(name, pattern, haystack)
//
// This benchmarks how fast a regular expression can report whether it matches
// a particular haystack. If the regex doesn't match, then the benchmark fails.
// Regexes are compiled exactly once.
//
// name is an identifier for the benchmark.
//
// pattern should be a &'static str representing the regular expression.
//
// haystack should be a String.
macro_rules! bench_match {
    ($module:ident, $name:ident, $pattern:expr, $haystack:expr) => {
        bench_is_match!($module, $name, true, regex!($pattern), $haystack);
    };
}

// USAGE: bench_not_match!(name, pattern, haystack)
//
// This benchmarks how fast a regular expression can report whether it matches
// a particular haystack. If the regex matches, then the benchmark fails.
// Regexes are compiled exactly once.
//
// name is an identifier for the benchmark.
//
// pattern should be a &'static str representing the regular expression.
//
// haystack should be a String.
macro_rules! bench_not_match {
    ($module:ident, $name:ident, $pattern:expr, $haystack:expr) => {
        bench_is_match!($module, $name, false, regex!($pattern), $haystack);
    };
}

// USAGE: bench_is_match!(name, is_match, regex, haystack)
//
// This benchmarks how fast a regular expression can report whether it matches
// a particular haystack. If the regex match status doesn't match is_match,
// then the benchmark fails. Regexes are compiled exactly once.
//
// name is an identifier for the benchmark.
//
// is_match reports whether the regex is expected to match the haystack or not.
//
// regex should be a ::Regex.
//
// haystack should be a String.
macro_rules! bench_is_match {
    ($module:ident, $name:ident, $is_match:expr, $re:expr, $haystack:expr) => {
        wrap_libtest! {
            $module,
            fn $name(b: &mut Bencher) {
                // Why do we use lazy_static here? It seems sensible to just
                // compile a regex outside of the b.iter() call and be done with
                // it. However, it seems like Rust's benchmark harness actually
                // calls the entire benchmark function multiple times. This doesn't
                // factor into the timings reported in the benchmarks, but it does
                // make the benchmarks take substantially longer to run because
                // they're spending a lot of time recompiling regexes.
                let text = text!($haystack);
                b.iter(|| {
                    if $re.is_match(&text) != $is_match {
                        if $is_match {
                            panic!("expected match, got not match");
                        } else {
                            panic!("expected no match, got match");
                        }
                    }
                });
            }
        }
    };
}

// USAGE: bench_find!(name, pattern, count, haystack)
//
// This benchmarks how fast a regular expression can count all successive
// non-overlapping matches in haystack. If the count reported does not match
// the count given, then the benchmark fails.
//
// name is an identifier for the benchmark.
//
// pattern should be a &'static str representing the regular expression.
//
// haystack should be a String.
macro_rules! bench_find {
    ($module:ident, $name:ident, $pattern:expr, $count:expr, $haystack:expr) => {
        wrap_libtest! {
            $module,
            fn $name(b: &mut Bencher) {
                let re = regex!($pattern);
                let text = text!($haystack);
                b.iter(|| {
                    let count = re.find_iter(&text).count();
                    assert_eq!($count, count)
                });
            }
        }
    };
}

// USAGE: bench_captures!(name, pattern, groups, haystack);
//
// CONTRACT:
//   Given:
//     ident, the desired benchmarking function name
//     pattern : ::Regex, the regular expression to be executed
//     groups : usize, the number of capture groups
//     haystack : String, the string to search
//   bench_captures will benchmark how fast re.captures() produces
//   the capture groups in question.
macro_rules! bench_captures {
    ($module:ident, $name:ident, $pattern:expr, $count:expr, $haystack:expr) => {
        wrap_libtest! {
            $module,
            fn $name(b: &mut Bencher) {
                let re = $pattern;
                let text = text!($haystack);
                b.iter(|| {
                    match re.captures(&text) {
                        None => assert!(false, "no captures"),
                        Some(caps) => assert_eq!($count + 1, caps.len()),
                    }
                });
            }
        }
    };
}
