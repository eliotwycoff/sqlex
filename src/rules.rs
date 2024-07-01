pub trait Faking {
    fn fake(&self) -> String;
}
pub trait FromStr: std::fmt::Debug {
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait FromStrFaking: FromStr + Faking {}

#[derive(Debug)]
pub struct UnknownFaker(pub String);
impl Faking for UnknownFaker {
    fn fake(&self) -> String {
        fakeit::name::first().to_string()
    }
}

impl FromStr for UnknownFaker {
    fn from_str(s: &str) -> Option<Self> {
        Some(UnknownFaker(s.to_string()))
    }
}

impl<T: FromStr + Faking> FromStrFaking for T {}

#[macro_export]
macro_rules! faking {
    ($($module:ident, $field_name:ident);*;) => {
        use std::collections::HashMap;

        $(
            paste::paste! {
                #[derive(Debug)]
                pub struct [<$field_name:camel>](pub String);
            }

            paste::paste! {
                impl Faking for [<$field_name:camel>] {
                    fn fake(&self) -> String {
                        fakeit::$module::$field_name().to_string()
                    }
                }
            }

            paste::paste! {
                impl From<&str> for [<$field_name:camel>] {
                    fn from(val: &str) -> Self {
                        [<$field_name:camel>](val.to_string())
                    }
                }
            }

            paste::paste! {
                impl FromStr for [<$field_name:camel>] {
                    fn from_str(s: &str) -> Option<Self> {
                        Some([<$field_name:camel>]::from(s))
                    }
                }
            }
        )*

        pub fn get_struct_by_name(name: &str) -> Box<dyn FromStrFaking> {
            let mut map: HashMap<&str, fn(&str) -> Box<dyn FromStrFaking>> = HashMap::new();
            $(
                paste::paste! {
                    let key = stringify!([<$field_name:camel>]).to_lowercase();
                    map.insert(key.as_str(), |s: &str| {
                        Box::new([<$field_name:camel>]::from(s)) as Box<dyn FromStrFaking>
                    });
                }
            )*
            match map.get(name) {
                Some(f) => f(name),
                None => Box::new(UnknownFaker(name.to_string())),
            }
        }
    };
}

faking! {
    contact, email;
    contact, phone;
    company, company;

    datetime, month;
    datetime, year;
    datetime, day;
    datetime, hour;
    datetime, minute;
    datetime, second;
    datetime, timezone;

    internet, domain_name;
    internet, http_method;
    internet, ipv4_address;
    internet, ipv6_address;
    internet, username;

    job, title;
    job, level;

    name, first;
    name, last;
    name, prefix;
    name, suffix;
    name, full;

    person, ssn;
    person, gender;

    unique, uuid_v4;

    words, word;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_struct_by_name() {
        let name = "first";
        let struct_by_name = get_struct_by_name(name);
        assert_eq!(
            format!("{:?}", struct_by_name),
            format!("{:?}", First::from_str("first").unwrap())
        );
    }
}
