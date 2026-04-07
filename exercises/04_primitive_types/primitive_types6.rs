fn main() {
    // You can optionally experiment here.
    let tupla_tipos_variados = (171, true, "Johny", 4.20);
    println!("{}", tupla_tipos_variados.0);
    println!("{}", tupla_tipos_variados.1);
    println!("{}", tupla_tipos_variados.2);
    println!("{}", tupla_tipos_variados.3);
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;

        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
