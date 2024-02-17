
pub fn add_one(x:i32)->i32{
    x+1
}
#[cfg(test)]
pub mod test_cases{
    use super::*;
    use crate::employee;

    #[test]
    pub fn test_one(){
        assert_eq!(employee(),(2,1,15));
        // assert_eq!(add_one(1),3);
    }
    
}