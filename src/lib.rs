//! # match_to_str
//!
//! Converts the pattern part of the `match` expression into a `&'static str`.  
//! It is mainly intended to be used when you want to convert a variable name into a string as is.
//!
//! ```rust
//! use match_to_str::match_to_str;
//!
//! const CAT: u8 = 1;
//! const DOG: u8 = 2;
//!
//! fn main() {
//!     let animal = match_to_str!(1 => {
//!         CAT,
//!         DOG,
//!         _,
//!     });
//!     assert_eq!(animal, "CAT");
//! }
//! ```

/// Converts the pattern part of the `match` expression into a `&'static str`.
#[macro_export]
macro_rules! match_to_str {
    { $x:expr => { $($val:pat),+ $(,)? } } => {
        match $x {
            $($val => stringify!($val),)*
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    mod nakano {
        pub const ICHIKA:  u8 =  1;
        pub const NINO:    u8 =  2;
        pub const MIKU:    u8 =  3;
        pub const YOTSUBA: u8 =  4;
        pub const ITSUKI:  u8 =  5;
    }

    fn nakano_to_str(nakano: u8) -> &'static str {
        use nakano::*;
        match_to_str!(nakano => {
            ICHIKA,
            NINO,
            MIKU,
            YOTSUBA,
            ITSUKI,
            _,
        })
    }

    trait NakanoToStr {
        fn to_str(&self) -> &'static str;
    }

    impl NakanoToStr for u8 {
        fn to_str(&self) -> &'static str {
            use nakano::*;
            match_to_str!(*self => {
                ICHIKA,
                NINO,
                MIKU,
                YOTSUBA,
                ITSUKI,
                _,
            })
        }
    }

    enum Yamada {
        Taro,
        Jiro,
        Saburo,
        Naoko, // お前のやったことは全部お見通しだ!
    }

    #[test]
    fn test_const() {
        use nakano::*;
        let tests = [
            (ICHIKA,  "ICHIKA"),
            (NINO,    "NINO"),
            (MIKU,    "MIKU"),
            (YOTSUBA, "YOTSUBA"),
            (ITSUKI,  "ITSUKI"),
        ];
        for (nakano, expect) in tests {
            assert_eq!(nakano_to_str(nakano), expect);
        }
    }

    #[test]
    fn test_trait() {
        use nakano::*;
        let tests = [
            (ICHIKA,  "ICHIKA"),
            (NINO,    "NINO"),
            (MIKU,    "MIKU"),
            (YOTSUBA, "YOTSUBA"),
            (ITSUKI,  "ITSUKI"),
        ];
        for (nakano, expect) in tests {
            assert_eq!(nakano.to_str(), expect);
        }
    }

    #[test]
    fn test_not_covered() {
        const ICHIRO:        u8 =  101;
        const NINOMIYA:      u8 =  102;
        const MILK:          u8 =  103;
        const YOSHIKI:       u8 =  104;
        const ITSUKIHIROSHI: u8 =  105;
        let tests = [
            (ICHIRO,        "_"),
            (NINOMIYA,      "_"),
            (MILK,          "_"),
            (YOSHIKI,       "_"),
            (ITSUKIHIROSHI, "_"),
        ];
        for (may_nakano, expect) in tests {
            assert_eq!(may_nakano.to_str(), expect);
        }
    }

    #[test]
    fn test_enum() {
        use Yamada::*;
        let tests = [
            (Taro,   "Taro"),
            (Jiro,   "Jiro"),
            (Saburo, "Saburo"),
            (Naoko,  "Naoko"),
        ];
        for (yamada, expect) in tests {
            let actual = match_to_str!(yamada => {
                Taro,
                Jiro,
                Saburo,
                Naoko,
            });
            assert_eq!(actual, expect);
        }
    }

    #[test]
    fn test_num() {
        let tests = [
            (1, "1"),
            (2, "2"),
            (3, "3 | 4"),
            (4, "3 | 4"),
            (5, "5 | 6 | 7"),
            (8, "_"),
        ];
        for (num, expect) in tests {
            let actual = match_to_str!(num => {
                1,
                2,
                3 | 4,
                5|6|7,
                _,
            });
            assert_eq!(actual, expect);
        }
    }

    #[test]
    fn test_str() {
        let tests = [
            ("cat",  r#""cat""#),
            ("dog",  r#""dog""#),
            ("bear", r#""bear""#),
            ("Cat",  "_"),
        ];
        for (str, expect) in tests {
            let actual = match_to_str!(str => {
                "cat",
                "dog",
                "bear",
                _,
            });
            assert_eq!(actual, expect);
        }
    }
}
