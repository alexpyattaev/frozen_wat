pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg_attr(feature = "frozen-abi", macro_use)]
#[cfg(feature = "frozen-abi")]
extern crate solana_frozen_abi_macro;

#[cfg_attr(feature = "frozen-abi", derive(AbiExample))]
struct Lol{
x:i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
