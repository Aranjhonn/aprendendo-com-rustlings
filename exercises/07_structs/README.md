# Structs

Rust has three struct types: a classic C struct, a tuple struct, and a unit struct.

## Further information

- [Structures](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

---

## Meu Diário de Aprendizado

**`structs1.rs`**
- **O Problema:** Definir e instanciar os três tipos de structs que existem no Rust.
- **O que aprendi:**
  - **Regular Struct** — Campos nomeados entre `{}`. Acesso via `.nome_do_campo`.
  - **Tuple Struct** — Campos posicionais entre `()`. Acesso via `.0`, `.1`, `.2` (idêntico a tuplas).
  - **Unit Struct** — Sem campos, apenas o nome. Útil como marcador ou para implementar traits.
- **Pegadinha:** Tentei usar `println!(variavel, ...)` passando uma variável como primeiro argumento da macro. O Rust exige que o primeiro argumento do `println!` seja sempre uma **string literal** (texto fixo entre aspas no código). Corrigi com `println!("{} {}", variavel, "texto")`.

**`structs2.rs`**
- **O Problema:** Criar uma nova `Order` baseada num template sem precisar reescrever todos os 7 campos manualmente.
- **O que aprendi:** A **Struct Update Syntax** (`..template`). Ao criar uma nova instância, posso definir apenas os campos que quero mudar e usar `..nome_do_molde` para copiar todo o restante automaticamente. Extremamente útil para structs com muitos campos.

**`structs3.rs`**
- **O Problema:** Adicionar lógica (métodos) a uma struct usando blocos `impl`.
- **O que aprendi:** O bloco `impl NomeDaStruct { }` permite criar **métodos** que operam sobre os dados da própria struct. O parâmetro `&self` dentro de um método é uma referência à instância que chamou o método. Já funções sem `&self` (como `new`) são chamadas de **funções associadas** e são acessadas com `::` (ex: `Package::new(...)`).
- **Pegadinha:** Tentei passar `0.50` (float) para `get_fees` que esperava `u32`. O compilador: *"expected u32, found floating-point number"*. Rust não converte tipos automaticamente!

---

## 🧪 Experimentos no `main()`

**`structs1.rs`** — Instanciando e imprimindo os 3 tipos de struct:
```rust
// Regular Struct (campos nomeados)
let marrom = ColorRegularStruct { green: 150, red: 75, blue: 0 };
println!("Imprimindo cores: {}", marrom.green);

// Tuple Struct (campos por índice)
let valores_tuple = ColorTupleStruct(150, 75, 0);
println!("Imprimindo valores Tuple: {}", valores_tuple.0);

// Unit Struct (sem campos, usa {:?} para imprimir)
let unit_struct = UnitStruct;
let mensagem = format!("{unit_struct:?}, Ainda estou aprendendo a usar");
println!("{} {}", mensagem, "e aprenderei em breve");
```

**`structs2.rs`** — Usando a Struct Update Syntax (`..molde`):
```rust
let molde = create_order_template();

let nova_order = Order {
    name: String::from("Aranjhonn desenvolvedor_rust"),
    count: 27,
    ..molde  // Copia year, made_by_phone, made_by_mobile, made_by_email, item_number
};

println!("{:?}", nova_order);
// Saída: Order { name: "Aranjhonn desenvolvedor_rust", year: 2019, ..., count: 27 }
```

**`structs3.rs`** — Usando métodos com `impl` e `&self`:
```rust
let pacote = Package::new(String::from("Holanda"), String::from("Brasil"), 50);
println!("{:?}", pacote.is_international()); // true (países diferentes)
println!("{}", pacote.get_fees(5));           // 250 (50g × 5 centavos)
```
