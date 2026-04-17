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
