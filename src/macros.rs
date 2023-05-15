/// # Usages
///
/// ```rust
/// use tracing_ext::trace;
///
/// #[derive(Debug, thiserror::Error)]
/// enum Error {
///     #[error("")]
///     A,
///     #[error("")]
///     B(String),
/// }
///
/// fn do_something() -> Result<(), Error> {
///     let result: Result<(), String> = Ok(());
///     match result {
///         Ok(s) => s,
///         // Use:
///         Err(err) => return trace!(Err(Error::A)),
///         // Equivalent code:
///         Err(err) => {
///             let err = Error::A;
///             tracing::error!("{}", err);
///             return Err(err);
///         }
///     };
///
///     // Using without constructor
///     None.ok_or(trace!(Error::A, "b error: {}", "something"))?;
///     // Equivalent code:
///     None.ok_or({
///         tracing::error!("b error: {}", "something");
///         Error::A
///     })?;
///
///     // Using with constructor
///     result.clone().map_err(trace!(Error::B))?;
///     // Equivalent code:
///     result.map_err(|err| {
///         let err = Error::B(err);
///         tracing::error!("{}", err);
///         err
///     })?;
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! trace {
    (Err($err: expr)) => {{
        let err = $err;
        tracing::error!("{}", err);
        Err(err)
    }};
    (|| $err: expr) => {{
        || {
            let err = $err;
            tracing::error!("{}", err);
            err
        }
    }};
    ($ret: expr, $($err:expr),+) => {{
        tracing::error!($($err),+);
        $ret
    }};
    ($err_kind: path) => {{
        |err| {
            let err = $err_kind(err);
            tracing::error!("{}", err);
            err
        }
    }};
}
