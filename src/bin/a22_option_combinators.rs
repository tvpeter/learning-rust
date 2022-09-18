/**
 * Topic: option combinators
 *
 * Requirements:
 * Use combinators as described in the functions:
 * part_1, part_2, and part_3
 *note
 use cargo test --bin a22_option_combinators
 */

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None,
    }
}

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn part_1() -> bool {
    /*
     * we are checking whether or not this particular user has an access level.
     * the "admin" user does have an access level
     * note: use is_some or is_none
     */

    maybe_access("admin").is_some()
}

fn part_2() -> Option<Access> {
    /*
    "root" is equivalent to Access::Admin but it is not listed in the maybe_access fn,
    Note: use or_else and root()
     */

    maybe_access("root").or_else(||root())
}

fn part_3() -> Access {
    /*
    "Alice" is not a listed user, so she will be a guest.
    Note: use unwrap_or_else
    */

    maybe_access("Alice").unwrap_or_else(||Access::Guest)
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_part_1() {
        assert_eq!(part_1(), true, "Admins have an access level");
    }

    #[test]
    fn check_part_2() {
        assert_eq!(
            part_2(),
            Some(Access::Admin),
            "Root users have Admin access"
        );
    }

    #[test]
    fn check_part_3(){
        assert_eq!(part_3(), Access::Guest, "Alice is a guest");
    }
}
