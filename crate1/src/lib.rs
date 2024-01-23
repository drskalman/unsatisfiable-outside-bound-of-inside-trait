pub trait Trait1 {

}

pub struct Struct1;

impl Trait1 for Struct1{}

#[cfg(test)]
mod tests {
    use super::Struct1;
    use crate2::Enum1;    

    #[test]
    fn it_works() {
        let my_enum : Enum1<Struct1>;
    }
}
