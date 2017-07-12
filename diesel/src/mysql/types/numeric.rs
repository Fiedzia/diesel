#[cfg(feature="bigdecimal")]
pub mod bigdecimal {
    extern crate num_traits;
    extern crate num_bigint;
    extern crate num_integer;
    extern crate bigdecimal;

    use std::error::Error;
    use std::io::prelude::*;
    use std::str::{self, FromStr};

    use mysql::Mysql;

    pub use self::bigdecimal::BigDecimal;

    use types::{self, FromSql, ToSql, IsNull};


    impl ToSql<types::Numeric, Mysql> for BigDecimal {
        fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error + Send + Sync>> {
            // Mysql representats decimal type as char[]
            out.write(&format!("{}", *self).into_bytes()).map(|_| IsNull::No).map_err(|e| e.into())
        }
    }

    impl FromSql<types::Numeric, Mysql> for BigDecimal {
        fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error+Send+Sync>> {
            //_ numeric
            match str::from_utf8(not_none!(bytes)) {
                Ok(s) => BigDecimal::from_str(s)
                            .map_err(|e| e.into()),
                Err(e) => Err(Box::new(e) as Box<Error + Send + Sync>)
            }
       }
    }
}
