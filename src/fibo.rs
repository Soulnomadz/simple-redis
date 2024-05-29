fn fibo(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        n => fibo(n-1) + fibo(n-2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    // #[should_panic]
    fn fibo_0() {
        assert_eq!(fibo(0), 0);
    }  

    #[test]
    fn fibo_01() {
        assert_eq!(fibo(1), 1);
    }
    
    #[test]
    fn fibo_02() {
        assert_eq!(fibo(2), 1);
    }

    #[test]
    fn fibo_03() {
        assert_eq!(fibo(3), 2);
    }
    
    #[test]
    fn fibo_05() {
        assert_eq!(fibo(5), 5);
    }
    
    #[test]
    fn fibo_10() {
        assert_eq!(fibo(10), 55);
    }    
}
