fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.to_vec(); // Cria um vetor novo a partir da referência

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let vetor = vec![0, 2, 4, 6, 8, 10];

    let vetor2 = fill_vec(&vetor);

    println!("Primeiro vetor: {:?}", vetor);
    println!("Segundo vetor: {:?}", vetor2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
