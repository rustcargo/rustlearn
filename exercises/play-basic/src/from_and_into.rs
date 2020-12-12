use std::{convert::{From, TryFrom, TryInto}, ops::Deref};

#[derive(Debug, PartialEq)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<usize> for Number {
    fn from(item: usize) -> Self {
        Number { value: item as i32 }
    }
}

impl TryFrom<String> for Number {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(item) => Ok(Number { value: item }),
            Err(_) => Err(()),
        }
    }
}

impl Deref for Number {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_from_into() {
        let num = Number::from(123);
        assert_eq!(123, num.value);

        let count = 360;
        let num2: Number = count.into();
        assert_eq!(count, num2.value);

        let count2: usize = 100;
        let num3: Number = count2.into();
        assert_eq!(count2 as i32, num3.value);

        let result = Number::try_from(String::from("123"));
        assert_eq!(result, Ok(Number { value: 123 }));
        let str = String::from("22");
        let num4: Result<Number, ()> = str.try_into();
        assert_eq!(num4, Ok(Number { value: 22 }));

        let dr = Number{value: 22};
        assert_eq!(*(dr.deref()), 22);
    }
}
