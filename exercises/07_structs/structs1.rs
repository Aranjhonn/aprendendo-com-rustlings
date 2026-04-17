struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    green: u8,
    red: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
    let marrom = ColorRegularStruct {
        green: 150,
        red: 75,
        blue: 0,
    };

    println!("Imprimindo cores: {}", marrom.green);
    println!("Imprimindo cores: {}", marrom.red);
    println!("Imprimindo cores: {}", marrom.blue);

    println!("=======================");

    let valores_tuple = ColorTupleStruct(150, 75, 0);

    println!("Imprimindo valores Tuple: {}", valores_tuple.0);
    println!("Imprimindo valores Tuple: {}", valores_tuple.1);
    println!("Imprimindo valores Tuple: {}", valores_tuple.2);

    println!("=======================");

    let unit_struct = UnitStruct;

    let mensagem = format!("{unit_struct:?}, Ainda estou aprendendo a usar");

    println!("{} {}", mensagem, "e aprenderei em breve");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct {
            green: 255,
            red: 0,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
