//! **raiment** provides application-level helpers influenced by
//! C-family scope techniques.
//!
//! The crate is `no_std` and depends only on `core`.
//!
//! The first helper is a block-scoped cleanup guard. The usual call
//! is [`on_drop!`]: the closure runs when its binding leaves the enclosing
//! block, including when that block unwinds. It does not wait until the
//! function returns.
//!
//! The design follows **STLSoft**'s `scoped_lambda` (and is related to
//! **STLSoft**'s `scoped_handle<>`).
//!
//! ```rust
//! let ran = std::cell::Cell::new(false);
//!
//! {
//!     raiment::on_drop! {
//!         ran.set(true);
//!     }
//!
//!     assert!(!ran.get());
//! }
//!
//! assert!(ran.get());
//! ```
//!
//! Name the guard with [`on_drop()`] only when you need
//! [`OnDropGuard::defuse`] to drop it without running the closure.
//! [`on_drop!`] hides its binding, so that guard cannot be defused.

#![no_std]

#[cfg(test)]
extern crate std;

mod guard;

pub use self::guard::{
    on_drop,
    OnDropGuard,
};


/// Runs the body at the end of the enclosing block.
///
/// Invoke the macro with braces. The braces are the callback. A
/// leading `||` is optional, for a programmer who wants the closure written
/// out; otherwise the macro supplies `||`. [`on_drop()`] always takes a
/// closure, so that call site keeps `||`.
///
/// The design follows **STLSoft**'s `scoped_lambda` (and is related to
/// **STLSoft**'s `scoped_handle<>`).
///
/// The guard runs on normal exit from that block and on unwind. It does not
/// wait until the enclosing function returns. Two uses in one block both
/// run, later declaration first. Each expansion binds its own identifier,
/// so the bindings do not shadow each other.
///
/// The expansion is a real `let` of a hidden name. It is not
/// `let _ = ...`. The wildcard would drop the guard at the end of the
/// statement. The hidden binding cannot be named, so the guard cannot be
/// defused; use [`on_drop()`] when you need [`OnDropGuard::defuse`].
///
/// A closure that panics during an already-unwinding drop aborts the
/// process.
#[macro_export]
macro_rules! on_drop {
    (|| $body:expr $(;)?) => {
        let _guard = $crate::on_drop(|| $body);
    };
    ($($body:tt)*) => {
        let _guard = $crate::on_drop(|| {
            $($body)*
        });
    };
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::{
        cell::{
            Cell,
            RefCell,
        },
        panic,
        string::String,
        vec::Vec,
    };

    use super::{
        on_drop,
        OnDropGuard,
    };

    fn push<'a>(
        log : &'a RefCell<Vec<i32>>,
        value : i32,
    ) -> OnDropGuard<impl FnOnce() + 'a> {
        on_drop(move || {
            log.borrow_mut().push(value);
        })
    }

    #[test]
    fn TEST_on_drop_MACRO_RUNS_AFTER_A_LATER_STATEMENT() {
        let ran = Cell::new(false);

        {
            crate::on_drop! {
                ran.set(true);
            }

            assert!(!ran.get());
        }

        assert!(ran.get());
    }

    #[test]
    fn TEST_TWO_on_drop_MACROS_RUN_IN_REVERSE_ORDER_OF_DECLARATION() {
        let log = RefCell::new(Vec::new());

        {
            crate::on_drop! {
                log.borrow_mut().push(1);
            }
            crate::on_drop! {
                log.borrow_mut().push(2);
            }

            assert!(log.borrow().is_empty());
        }

        assert_eq!(&[2, 1][..], &log.borrow()[..]);
    }

    #[test]
    fn TEST_THREE_on_drop_MACROS_RUN_IN_REVERSE_ORDER() {
        let log = RefCell::new(Vec::new());

        {
            crate::on_drop! {
                log.borrow_mut().push(1);
            }
            crate::on_drop! {
                log.borrow_mut().push(2);
            }
            crate::on_drop! {
                log.borrow_mut().push(3);
            }
        }

        assert_eq!(&[3, 2, 1][..], &log.borrow()[..]);
    }

    #[test]
    #[allow(unused_must_use)]
    fn TEST_UNBOUND_on_drop_RUNS_BEFORE_THE_FOLLOWING_STATEMENT() {
        let n = Cell::new(0);

        on_drop(|| {
            n.set(n.get() + 1);
        });

        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_BOUND_on_drop_WAITS_UNTIL_THE_END_OF_THE_BLOCK() {
        let n = Cell::new(0);

        {
            let _guard = on_drop(|| {
                n.set(n.get() + 1);
            });

            assert_eq!(0, n.get());
        }

        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_defuse_PREVENTS_THE_CALL() {
        let n = Cell::new(0);

        {
            let guard = on_drop(|| {
                n.set(n.get() + 1);
            });

            guard.defuse();
            assert_eq!(0, n.get());
        }

        assert_eq!(0, n.get());
    }

    #[test]
    fn TEST_defuse_AND_DROP_DO_NOT_BOTH_RUN_THE_CLOSURE() {
        let n = Cell::new(0);

        let guard = on_drop(|| {
            n.set(n.get() + 1);
        });

        guard.defuse();

        assert_eq!(0, n.get());
    }

    #[test]
    fn TEST_DROP_RUNS_THE_CLOSURE_ONLY_ONCE() {
        let n = Cell::new(0);

        {
            let guard = on_drop(|| {
                n.set(n.get() + 1);
            });

            drop(guard);
            assert_eq!(1, n.get());
        }

        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_GUARD_INSIDE_catch_unwind_HAS_ALREADY_RUN() {
        let n = Cell::new(0);

        let caught = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let _guard = on_drop(|| {
                n.set(n.get() + 1);
            });

            panic!("unwind the scope");
        }));

        assert!(caught.is_err());
        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_UNWIND_RUNS_GUARDS_IN_REVERSE_ORDER() {
        let log = RefCell::new(Vec::new());

        let caught = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            crate::on_drop! {
                || log.borrow_mut().push(1);
            }
            crate::on_drop! {
                || {
                    log.borrow_mut().push(2);
                }
            }

            panic!("unwind the scope");
        }));

        assert!(caught.is_err());
        assert_eq!(&[2, 1][..], &log.borrow()[..]);
    }

    #[test]
    fn TEST_GUARD_OUTSIDE_THE_PANICKING_SCOPE_HAS_NOT_RUN() {
        let outer = Cell::new(0);
        let _guard = on_drop(|| {
            outer.set(outer.get() + 1);
        });

        let caught = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            panic!("leave the outer guard in place");
        }));

        assert!(caught.is_err());
        assert_eq!(0, outer.get());
    }

    #[test]
    fn TEST_PANIC_FROM_A_NORMAL_DROP_CAN_BE_CAUGHT() {
        let earlier = Cell::new(false);

        let caught = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let _first = on_drop(|| {
                panic!("from drop");
            });
            let _second = on_drop(|| {
                earlier.set(true);
            });
        }));

        assert!(caught.is_err());
        assert!(earlier.get());
    }

    #[test]
    fn TEST_MOVED_GUARD_RUNS_AT_THE_END_OF_THE_NESTED_BLOCK() {
        fn timing() -> Vec<i32> {
            let log = RefCell::new(Vec::new());
            let guard = on_drop(|| {
                log.borrow_mut().push(1);
            });

            log.borrow_mut().push(0);

            {
                let _guard = guard;

                log.borrow_mut().push(2);
            }

            log.borrow_mut().push(3);
            log.into_inner()
        }

        assert_eq!(&[0, 2, 1, 3][..], &timing()[..]);
    }

    #[test]
    fn TEST_GUARD_MOVED_OUT_OF_A_BLOCK_WAITS_FOR_THE_OUTER_SCOPE() {
        let n = Cell::new(0);
        let guard = {
            let guard = on_drop(|| {
                n.set(n.get() + 1);
            });

            assert_eq!(0, n.get());
            guard
        };

        assert_eq!(0, n.get());
        drop(guard);
        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_GUARD_IN_A_LOOP_RUNS_EACH_ITERATION() {
        let seen = RefCell::new(Vec::new());

        for i in 0..4 {
            crate::on_drop! {
                seen.borrow_mut().push(i);
            }

            assert_eq!(i as usize, seen.borrow().len());
        }

        assert_eq!(&[0, 1, 2, 3][..], &seen.borrow()[..]);
    }

    #[test]
    fn TEST_EARLY_RETURN_RUNS_THE_GUARD_BEFORE_THE_CALLER_CONTINUES() {
        fn early(
            flag : &mut i32,
            leave : bool,
        ) -> i32 {
            let _guard = on_drop(|| {
                *flag += 1;
            });

            if leave {
                return 7;
            }

            0
        }

        let mut flag = 0;

        assert_eq!(7, early(&mut flag, true));
        assert_eq!(1, flag);
    }

    #[test]
    fn TEST_EXPLICIT_DROP_RUNS_THE_GUARD_BEFORE_LATER_STATEMENTS() {
        let n = Cell::new(0);
        let guard = on_drop(|| {
            n.set(n.get() + 1);
        });

        assert_eq!(0, n.get());
        drop(guard);
        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_OVERWRITING_A_NAMED_GUARD_RUNS_THE_PREVIOUS_CLOSURE() {
        let log = RefCell::new(Vec::new());
        #[allow(unused_assignments)]
        let mut guard = push(&log, 1);

        guard = push(&log, 2);

        assert_eq!(&[1][..], &log.borrow()[..]);
        drop(guard);
        assert_eq!(&[1, 2][..], &log.borrow()[..]);
    }

    #[test]
    fn TEST_on_drop_CONSUMES_A_NON_COPY_CAPTURE() {
        let sink = RefCell::new(String::new());

        {
            let owned = String::from("gone");
            let sink = &sink;
            let _guard = on_drop(move || {
                sink.borrow_mut().push_str(&owned);
            });

            assert!(sink.borrow().is_empty());
        }

        assert_eq!("gone", sink.borrow().as_str());
    }

    #[test]
    fn TEST_INNER_BLOCK_DOES_NOT_RUN_THE_OUTER_GUARD() {
        let log = RefCell::new(Vec::new());

        {
            crate::on_drop! {
                log.borrow_mut().push(1);
            }

            {
                crate::on_drop! {
                    log.borrow_mut().push(2);
                }
                log.borrow_mut().push(0);
            }

            assert_eq!(&[0, 2][..], &log.borrow()[..]);
        }

        assert_eq!(&[0, 2, 1][..], &log.borrow()[..]);
    }

    #[test]
    fn TEST_NAMED_GUARD_AND_MACRO_RUN_LATER_DECLARATION_FIRST() {
        let log = RefCell::new(Vec::new());

        {
            let _guard = on_drop(|| log.borrow_mut().push(1));

            crate::on_drop! {
                log.borrow_mut().push(2);
            }
        }

        assert_eq!(&[2, 1][..], &log.borrow()[..]);
    }

    #[test]
    fn TEST_CALLING_ANOTHER_FUNCTION_DOES_NOT_RUN_THE_GUARD() {
        fn callee() {
        }

        let n = Cell::new(0);

        {
            let _guard = on_drop(|| {
                n.set(n.get() + 1);
            });

            callee();
            assert_eq!(0, n.get());
        }

        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_CLOSURE_SEES_MUTATIONS_MADE_AFTER_THE_GUARD_WAS_CREATED() {
        let n = Cell::new(1);

        {
            let _guard = on_drop(|| {
                n.set(n.get() * 10);
            });

            n.set(n.get() + 2);
            assert_eq!(3, n.get());
        }

        assert_eq!(30, n.get());
    }

    #[test]
    fn TEST_ONLY_THE_TAKEN_BRANCH_RUNS_ITS_GUARD() {
        let n = Cell::new(0);

        if true {
            crate::on_drop! {
                n.set(n.get() + 1);
            }

            assert_eq!(0, n.get());
        } else {
            crate::on_drop! {
                n.set(n.get() + 10);
            }
        }

        assert_eq!(1, n.get());
    }

    #[test]
    fn TEST_defuse_OF_ONE_GUARD_LEAVES_THE_OTHER() {
        let log = RefCell::new(Vec::new());

        {
            let silenced = on_drop(|| log.borrow_mut().push(1));

            crate::on_drop! {
                || log.borrow_mut().push(2)
            }
            silenced.defuse();
        }

        assert_eq!(&[2][..], &log.borrow()[..]);
    }
}

/* ///////////////////////////// end of file //////////////////////////// */
