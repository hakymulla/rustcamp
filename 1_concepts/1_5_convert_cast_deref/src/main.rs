use std::any::{type_name, type_name_of_val};
use std::ops::{Deref, DerefMut, Mul};
use std::fmt::Debug;
use rand::Rng;
use std::collections::HashSet;

#[derive(Debug)]
struct EmailString(String);

impl Deref for EmailString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EmailString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl From<String> for EmailString {
    fn from(value: String) -> Self {
        EmailString(value)
    }
}

impl Into<String> for EmailString {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug, PartialEq)]
struct Random<T> (T, T, T);

impl<T: Debug> Deref for Random <T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let num = rand::rng().random_range(0..3);
        match num {
            0 => &self.0,
            1 => &self.1,
            _ => &self.2,
        }
    }
}

impl<T: Debug> DerefMut for Random<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let email = EmailString("test@gmail.com".to_string());
    println!("Email Len: {:?}", email.len());

    let val = email.split_once('.');
    println!("Email Split: {:?}", val);

    //  let email_str: String = email.into();
    // println!("Email String: {:?}", email_str);

    let random: Random<u8> = Random(1, 5, 7);
    println!("Random **2 : {:?}", random.pow(2));
    println!("Random **2 : {:?}", random.pow(2));
    println!("Random **2 : {:?}", random.pow(2));

    println!("Random * 3 : {:?}", random.mul(3));
    println!("Random * 3 : {:?}", random.mul(3));
    println!("Random * 3 : {:?}", random.mul(3));

    let email = EmailString::from("test@gmail.com".to_string());
    println!("Email type: {:?}", type_name_of_val(&email));

    let ema = "test@test.com".to_string();
    let em: EmailString = ema.into();
    println!("EM: {:?}", em);
    

}


#[test]
fn test_email_string_split_once() {
    let email = EmailString("test@gmail.com".to_string());
    assert!(email.split_once('.') == Some(("test@gmail", "com")))
}

#[test]
fn test_email_string_from() {
    let email = EmailString::from("test@gmail.com".to_string());
    assert!(type_name_of_val(&email).contains("EmailString"));

}

#[test]
fn test_email_string_into() {
    let email = EmailString("test@gmail.com".to_string());
    let email_string: String = email.into();

    assert!(type_name_of_val(&email_string).contains("str"));
}


#[test]
fn test_random_deref() {
    let random = Random(1, 2, 3);
    let mut seen_values = HashSet::new();

    for _ in 0..100 {
        let value = *random;
        assert!([1, 2, 3].contains(&value), "Deref returned invalid value: {}", value);
        seen_values.insert(value);
    }

    assert_eq!(
        seen_values.len(),
        3,
        "Not all fields were selected: {:?}", seen_values
    );
}

#[test]
fn test_random_deref_string() {
    let random = Random("a".to_string(), "b".to_string(), "c".to_string());
    let mut seen_values = HashSet::new();

    for _ in 0..100 {
        let value = random.deref(); 
        assert!(
            ["a", "b", "c"].contains(&value.as_str()),
            "Deref returned invalid value: {}",
            value
        );
        seen_values.insert(value.as_str());
    }

    assert_eq!(
        seen_values.len(),
        3,
        "Not all fields were selected: {:?}", seen_values
    );
}