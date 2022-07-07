use serde::{Deserialize, Serialize};
use crate::challenge::Challenge;

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashChallenge {
    pub input: MD5HashCashInput
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

impl Challenge for MD5HashCashChallenge {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCashChallenge { input }
    }

    fn solve(&self) -> Self::Output {
        let mut answer = Self::Output {
            seed: 0,
            hashcode: "".to_string()
        };

        for seed in 0..=u64::MAX {
            let input = format!("{seed:0>16X}{}", self.input.message);
            let hashcode = format!("{:0>16X}", md5::compute(&input));
            let num_hashcode = u128::from_str_radix(&hashcode, 16).unwrap();
    
            let zeros = num_hashcode.leading_zeros();
            if zeros >= self.input.complexity {
                answer = Self::Output {
                    seed,
                    hashcode
                };
                break;
            }
        }
        answer
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let seed = answer.seed;
        let input = format!("{seed:0>16X}{}", self.input.message);
        let hashcode = format!("{:X}", md5::compute(&input));
        count_bits_to_zero(&hashcode) >= self.input.complexity && answer.hashcode == hashcode
    } 
}

fn count_bits_to_zero(hex_string: &str) -> u32 {
    let hex_value = u128::from_str_radix(hex_string, 16).unwrap();
    hex_value.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_challenge() -> MD5HashCashChallenge {
        MD5HashCashChallenge {
            input: MD5HashCashInput { 
                complexity: 9, 
                message: "hello".to_string() 
            }
        }
    }
    
    #[test]
    pub fn verify_should_return_true() {
        let challenge = create_challenge();
        let answer = MD5HashCashOutput {
            seed: 844,
            hashcode: "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string()
        };
        assert_eq!(true, challenge.verify(&answer));
    }

    #[test]
    pub fn solver_should_return_a_valid_seed() {
        let challenge = create_challenge();
        let answer = challenge.solve();
        assert_eq!(true, challenge.verify(&answer));
    }
}