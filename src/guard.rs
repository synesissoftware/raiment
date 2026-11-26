// src/guard.rs : definition of `OnDropGuard`

/// A guard that runs a closure when it is dropped.
///
/// An unused guard runs the closure immediately.
#[must_use = "an unused guard runs the closure immediately"]
pub struct OnDropGuard<F>
where
    F : FnOnce(),
{
    action : Option<F>,
}


impl<F> OnDropGuard<F>
where
    F : FnOnce(),
{
    /// Drops this guard without running its closure.
    ///
    /// `Drop` then sees no closure, so `defuse` and `Drop` do not
    /// both run it. A guard from [`crate::on_drop!`] cannot be
    /// defused: the macro hides the binding.
    pub fn defuse(mut self) {
        self.action.take();
    }
}


impl<F> Drop for OnDropGuard<F>
where
    F : FnOnce(),
{
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            action();
        }
    }
}


/// Builds a guard that runs `action` when the guard is dropped.
///
/// The closure runs on normal scope exit and on unwind. A closure
/// that panics while a drop is already unwinding aborts the process.
///
/// Bind the returned [`OnDropGuard`]. An unbound temporary runs at
/// the end of the statement, before the following statement. That is
/// the hazard. [`crate::on_drop!`] is the normal call site. Invoke
/// it with braces; a leading `||` is optional. The macro expands to
/// a real `let` binding, so the closure runs at the end of the
/// enclosing block. The design follows **STLSoft**'s `scoped_lambda`, and
/// is related to `scoped_handle<>`.
///
/// Do not write `let _ = on_drop(...)`. The wildcard pattern drops
/// the guard at the end of that statement, the same as leaving the
/// temporary unbound. `#[must_use]` on [`OnDropGuard`] does not catch
/// `let _`.
///
/// Call [`OnDropGuard::defuse`] to drop a named guard without running
/// the closure. [`crate::on_drop!`] hides its binding, so a guard
/// created by the macro cannot be defused.
pub fn on_drop<F>(action : F) -> OnDropGuard<F>
where
    F : FnOnce(),
{
    OnDropGuard {
        action : Some(action)
    }
}

/* ///////////////////////////// end of file //////////////////////////// */
