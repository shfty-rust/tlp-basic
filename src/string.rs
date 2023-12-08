use std::marker::PhantomData;

pub trait StringTrait {
    fn string() -> String;
}

impl<CAR, CDR> StringTrait for (CAR, CDR) where CAR: StringTrait, CDR: StringTrait {
    fn string() -> String {
        CAR::string() + &CDR::string()
    }
}

impl StringTrait for () {
    fn string() -> String {
        "".to_owned()
    }
}

pub struct Up<C>(PhantomData<C>);

impl<C> StringTrait for Up<C> where C: StringTrait {
    fn string() -> String {
        C::string().to_uppercase()
    }
}

#[macro_export]
macro_rules ! String {
    ($($char:ident)*) => {
        $crate::List![$(crate::char::$char),*]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_trait() {
        type StringType = String![H e l l o Comma Space W o r l d Exclamation];
        println!("{}", StringType::string());
    }
}
