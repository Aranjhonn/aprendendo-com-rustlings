# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

---

## Meu Diário de Aprendizado

**`strings1.rs`**
- **O Problema:** A função prometia retornar um `String`, mas retornava um `&str` literal (`"blue"`).
- **O que aprendi:** Como converter um `&str` para `String` usando `String::from()`. Existem dois tipos principais de strings em Rust: `&str` (fatia de texto, rápido e eficiente, geralmente literal) e `String` (texto dinâmico, dono dos dados no heap).

**`strings2.rs`**
- **O Problema:** A função esperava receber um `&str`, mas passamos um `String`.
- **O que aprendi:** Como converter um `String` em `&str` (emprestar). Basta usar o símbolo de referência `&`. Ou seja, `&word` empresta o `String` como um `&str`.

**`strings3.rs`**
- **O Problema:** Manipular strings de 3 formas diferentes (remover espaços, concatenar, substituir palavras).
- **O que aprendi:** A biblioteca padrão do Rust possui métodos super úteis:
  - `.trim()`: Remove espaços em branco do início e do fim. Retorna um `&str`.
  - `format!()`: Macro poderosa (similar ao `println!`) para criar e formatar novos `String`s.
  - `.replace("velho", "novo")`: Substitui ocorrências de uma palavra por outra em um texto e retorna um `String`.
- **Pegadinha:** Na função `replace_me`, acabei escrevendo `"toballons"` primeiro com erro de digitação, e o teste apontou na hora. O `format!` precisava de `" world!"` exato, e eu tinha esquecido da exclamação. E por último, eu criei uma variável intermediária no `compose_me`, mas depois percebi que poderia retornar o `format!` direto!

**`strings4.rs`**
- **O Problema:** Identificar, linha por linha, quais comandos do Rust geram uma referência (`&str`) e quais geram uma String dinâmica (alocada no Heap, tipo `String`).
- **O que aprendi:** 
  - Geração de `&str`: `"literal"` direto, `.trim()` e fatias de outras strings (`&String::from("abc")[0..1]`).
  - Geração de `String`: Métodos que criam novas cópias no heap, como `.to_string()`, `.to_owned()`, `.into()`, `.replace()`, `.to_lowercase()` e a macro `format!()`.

---

## 🧪 Experimentos no `main()`

**`strings3.rs`** — Usando os três métodos na prática:
```rust
// .trim() removeu os vários espaços que coloquei em volta do texto
println!("{}", trim_me("       Aranjhonn Desenvolvedor Rust       ")); 
// Saída: "Aranjhonn Desenvolvedor Rust"

// format!() juntou a minha frase com " world!"
println!("{}", compose_me("Aranjhonn Desenvolvedor Rust está dizendo: Olá,"));
// Saída: "Aranjhonn Desenvolvedor Rust está dizendo: Olá, world!"

// .replace("cars", "balloons")
// Observação engraçada: Eu passei a palavra "Porsche". Como "Porsche" não contém 
// a palavra "cars", o replace não mudou nada e me devolveu "Porsche" intacto!
println!("Em breve comprarei o meu {}", replace_me("Porsche"));
// Saída: "Em breve comprarei o meu Porsche"
```
