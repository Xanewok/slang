// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(unused_macros)]
macro_rules! scan_chars {
    ($stream:ident, $($char:literal),+) => {
        if $( $stream.next() == Some($char) )&&+ {
            Scan::Strict
        } else {
            $stream.undo();
            Scan::None
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_none_of {
    ($stream:ident, $($char:literal),+) => {
        if let Some(c) = $stream.next() {
            if $(c != $char)&&+ {
                Scan::Strict
            } else {
                $stream.undo();
                Scan::None
            }
        } else {
            $stream.undo();
            Scan::None
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_char_range {
    ($stream:ident, $from:literal..=$to:literal) => {
        if let Some(c) = $stream.next() {
            #[allow(clippy::manual_is_ascii_check)]
            if ($from..=$to).contains(&c) {
                Scan::Strict
            } else {
                $stream.undo();
                Scan::None
            }
        } else {
            $stream.undo();
            Scan::None
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_sequence {
    ($($scanner:expr),*) => {
        if $(($scanner).matched())&&* {
            // TODO: Handle the Ambiguous/Strict
            Scan::Strict
        } else {
            Scan::None
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_choice {
    ($stream:ident, $($scanner:expr),*) => {
        loop {
            let save = $stream.position();
            $(
                if ($scanner).matched() { break Scan::Strict }
                $stream.set_position(save);
            )*
            break Scan::None
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_zero_or_more {
    ($stream:ident, $scanner:expr) => {
        loop {
            let save = $stream.position();
            if !($scanner).matched() {
                $stream.set_position(save);
                break Scan::Strict;
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_one_or_more {
    ($stream:ident, $scanner:expr) => {{
        let mut count = 0;
        #[allow(clippy::redundant_else)]
        loop {
            let save = $stream.position();
            if !($scanner).matched() {
                if count < 1 {
                    break Scan::None;
                } else {
                    $stream.set_position(save);
                    break Scan::Strict;
                }
            }
            count += 1;
        }
    }};
}

#[allow(unused_macros)]
macro_rules! scan_optional {
    ($stream:ident, $scanner:expr) => {{
        let save = $stream.position();
        if !($scanner).matched() {
            $stream.set_position(save)
        }
        Scan::Strict
    }};
}

#[allow(unused_macros)]
macro_rules! scan_not_followed_by {
    ($stream:ident, $scanner:expr, $not_followed_by:expr) => {
        // TODO: Handle ambiguous/strict matches
        if ($scanner).matched()
            && ({
                let end = $stream.position();
                let following = $not_followed_by;
                $stream.set_position(end);
                !following.matched()
            })
        {
            Scan::Strict
        } else {
            Scan::None
        }
    };
}
