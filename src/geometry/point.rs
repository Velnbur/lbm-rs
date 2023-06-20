use crate::num;
use std;

pub trait AmbientDimension {
    fn ambient_dimension(&self) -> usize;
}

pub trait ObjectDimension {
    fn object_dimension(&self) -> usize;
}

pub trait Point:
    AmbientDimension + ObjectDimension + std::ops::Index<num>
{
}

#[cfg(test)]
mod tests {

    #[test]
    fn point_tests() {}
}
