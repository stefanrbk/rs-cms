macro_rules! err {
    (str => $s:tt) => { {
            Err($s)
        }
    };
    ($c:expr, $l:ident, $e:ident, $s:tt; str => $ss:tt) => { {
                $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &$s
            );
            Err($ss)
        }
    };
    ($c:expr, $l:ident, $e:ident, $s:tt, $($exprs:expr),*; str => $ss:tt) => { {
                $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &format!($s, $($exprs),*)
            );
            Err($ss)
        }
    };
    (inner => $c:expr, $l:ident, $e:ident, $s:tt; str => $ss:tt) => { {
            Context(Arc::new($c.clone())).signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &$s
            );
            Err($ss)
        }
    };
    (inner => $c:expr, $l:ident, $e:ident, $s:tt, $($exprs:expr),*; str => $ss:tt) => { {
            Context(Arc::new($c.clone())).signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &format!($s, $($exprs),*)
            );
            Err($ss)
        }
    };
    (io => $t:ident, $s:tt) => {
        Err(std::io::Error::new(std::io::ErrorKind::$t, $s))
    };
    ($c:expr, $l:ident, $e:ident, $s:tt; io => $t:ident, $ss:tt) => {{
        $c.signal_error(
            log::Level::$l,
            crate::state::ErrorCode::$e,
            $s
        );
        Err(std::io::Error::new(std::io::ErrorKind::$t, $ss))}
    };
    ($c:expr, $l:ident, $e:ident, $s:tt; io => $error:expr) => { {
                $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &$s
            );
            Err($error)
        }
    };
    ($c:expr, $l:ident, $e:ident, $s:tt, $($exprs:expr),*; io => $t:ident, $ss:tt) => { {
            $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &format!($s, $($exprs),*)
            );
            Err(std::io::Error::new(std::io::ErrorKind::$t, $ss))
        }
    };
    ($c:expr, $l:ident, $e:ident, $s:tt, $($exprs:expr),*; io => $error:expr) => { {
                $c.signal_error(
                log::Level::$l,
                crate::state::ErrorCode::$e,
                &format!($s, $($exprs),*)
            );
            Err($error)
        }
    };
}
