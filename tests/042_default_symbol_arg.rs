//! Regression: the `default` statement argument is a string (RFC 7950) whose
//! type depends on the leaf — values such as a time interval `00:00:15.0` or a
//! prefixed identityref `syslogtypes:local7` are valid bare (unquoted)
//! arguments containing `:`. Previously any such value collapsed the module
//! (real-world cases: standard `ietf-netconf-time`, DRAFT `ietf-syslog`).

mod test_utils;

use test_utils::str_to_ast;

fn ok(src: &str) {
    let tree = str_to_ast(src);
    assert!(
        !tree.root_node().has_error(),
        "expected no parse error in:\n{src}"
    );
}

#[test]
fn test_default_time_interval() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef time-interval {
        type string {
            pattern '\d{2}:\d{2}:\d{2}(\.\d+)?';
        }
    }
    leaf sched-max-future {
        type time-interval;
        default 00:00:15.0;
    }
}
    "#);
}

#[test]
fn test_default_prefixed_identityref() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    import syslogtypes {
        prefix syslogtypes;
    }
    leaf facility {
        type identityref {
            base syslogtypes:syslog-facility;
        }
        default syslogtypes:local7;
    }
}
    "#);
}

#[test]
fn test_default_numeric_and_quoted_still_work() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    leaf a { type uint16; default 8080; }
    leaf b { type decimal64 { fraction-digits 2; } default -3.25; }
    leaf c { type string; default "gin"; }
    leaf d { type enumeration { enum on; enum off; } default on; }
}
    "#);
}
