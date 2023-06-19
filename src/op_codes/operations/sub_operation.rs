use super::super::CodeOperation;
use crate::virtual_machine::ExecutionRuntime;
use ibig::UBig;

#[derive(Clone)]
pub struct SubOperation {}

impl CodeOperation for SubOperation {
    fn execute(&self, vm: &mut ExecutionRuntime, _word: Option<Vec<u8>>) {
        let hex_str1 = hex::encode(vm.stack.pop().unwrap());
        let hex_str2 = hex::encode(vm.stack.pop().unwrap());

        let num1 = UBig::from_str_radix(&hex_str1, 16).unwrap();
        let num2 = UBig::from_str_radix(&hex_str2, 16).unwrap();

        if num1 > num2 {
            let result = num1 - num2;
            vm.stack.push(result.to_be_bytes());
        } else if num2 > num1 {
            let result = num2 - num1;
            vm.stack.push(result.to_be_bytes());
        } else {
            vm.stack.push(vec![0x0]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sub() {
        let sub = SubOperation {};
        let stack: Vec<Vec<u8>> = vec![vec![0x2], vec![0x2]];
        let mut vm = ExecutionRuntime::new_with_stack(stack);
        sub.execute(&mut vm, None);
        assert_eq!(vm.stack, vec![vec![0x0]]);
    }
}
