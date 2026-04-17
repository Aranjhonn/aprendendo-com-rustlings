fn main() {
    // You can optionally experiment here.
    let mut vetor_mut1 = Vec::new();

    let vetor_mut2 = &mut vetor_mut1;

    vetor_mut2.push("João");

    let vetor_mut3 = &mut vetor_mut1;

    vetor_mut3.push("Pedro");

    println!("Valor final do vetor 1: {:?}", vetor_mut1);
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);

        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
