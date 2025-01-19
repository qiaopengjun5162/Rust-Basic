// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}

/*
在这段代码中，abs_all 函数的返回值不同是因为它的行为取决于输入数据的状态（是否为“拥有”或“借用”）以及是否需要对数据进行修改。以下是详细的解释：
1. Cow::Borrowed：
当输入数据是借用的（即通过引用传递），并且在函数内部没有对数据进行修改时，返回值为 Cow::Borrowed。
例如，在 reference_no_mutation 测试中，输入是一个只包含非负数的切片，因此没有进行任何修改，返回 Cow::Borrowed。
2. Cow::Owned：
当输入数据是借用的，但在函数内部需要对数据进行修改时，返回值为 Cow::Owned。在这种情况下，Cow 会克隆数据以便进行修改。
例如，在 reference_mutation 测试中，输入包含负数，函数需要将其转换为正数，因此返回 Cow::Owned。
3. 拥有数据的情况：
当输入数据是拥有的（即直接传递一个 Vec），如果没有进行修改，返回值仍然是 Cow::Owned。这是因为数据从一开始就是拥有的，不需要克隆。
在 owned_no_mutation 测试中，输入是一个 Vec，没有进行修改，因此返回 Cow::Owned。
4. 修改拥有数据：
如果输入数据是拥有的，并且在函数内部进行了修改，返回值仍然是 Cow::Owned，因为数据的所有权没有改变。
在 owned_mutation 测试中，输入包含负数，函数将其转换为正数，因此返回 Cow::Owned。
总结来说，返回值的不同主要取决于输入数据的状态（借用或拥有）以及是否需要对数据进行修改。这样设计的目的是为了在需要时尽量避免不必要的克隆，提高性能。
*/
