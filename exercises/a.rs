fn main(){
    
    pub struct WrappingU32{
        inner: u32;
    }
    
    impl MaybeZero for WrappingU32{
        fn is_zero(self)-> bool {
            self.inner == 0
        }
    }

    let a = WrappingU32{inner: 0};
    println!("{}", a.is_zero());

}
