# Enums

Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

## Further information

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Pattern syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)

---

## Meu Diário de Aprendizado

**`enums1.rs`**
- **O Problema:** Criar um enum simples com variantes sem dados.
- **O que aprendi:** No Rust, enums podem ter múltiplas **variantes** (como categorias). Cada variante é acessada com `NomeDoEnum::Variante`. Adicionando `#[derive(Debug)]` posso imprimir as variantes com `{:?}`.

**`enums2.rs`**
- **O Problema:** Criar variantes de enum que **carregam dados** de tipos diferentes.
- **O que aprendi:** Cada variante de um enum pode guardar dados de formas diferentes:
  - **Struct-like:** `Resize { width: u32, height: u32 }` — com campos nomeados
  - **Tuple-like:** `Move(Point)` ou `ChangeColor(u8, u8, u8)` — com dados posicionais
  - **String:** `Echo(String)` — carregando uma String
  - **Sem dado:** `Quit` — apenas um marcador
- **Conceito extra:** Também adicionamos um `impl Message` com um método `.call()` que imprime a mensagem usando `{self:?}`.

**`enums3.rs`**
- **O Problema:** Usar `match` para processar cada variante do enum, chamando métodos diferentes dependendo da mensagem recebida.
- **O que aprendi:** O **`match`** é a ferramenta do Rust para "pattern matching". Ele obriga a tratar **todas** as variantes do enum — se esquecer uma, o compilador reclama. Isso garante que nenhum caso seja ignorado acidentalmente.
- **Pegadinha:** Tentei imprimir `state.position` com `{:?}`, mas a struct `Point` não tinha `#[derive(Debug)]`. O compilador sugeriu exatamente a solução: adicionar a anotação. Resolvi de duas formas: imprimindo campos individuais (`position.x`, `position.y`) e também adicionando `#[derive(Debug)]` ao `Point` para imprimir a struct inteira.

---

## 🧪 Experimentos no `main()`

**`enums1.rs`** — Imprimindo todas as variantes do enum:
```rust
println!("{:?}", Message::Resize);
println!("{:?}", Message::Move);
println!("{:?}", Message::Echo);
println!("{:?}", Message::ChangeColor);
println!("{:?}", Message::Quit);
```

**`enums2.rs`** — Criando variantes com dados e iterando com `for`:
```rust
let messages = [
    Message::Resize { width: 10, height: 30 },
    Message::Move(Point { x: 10, y: 15 }),
    Message::Echo(String::from("hello world")),
    Message::ChangeColor(200, 255, 255),
    Message::Quit,
];

for message in &messages {
    message.call();
}
```

**`enums3.rs`** — Motor de estado usando `match` e `process`:
```rust
let mut state = State {
    width: 5,
    height: 5,
    position: Point { x: 1, y: 2 },
    message: String::from("Desenvolvedor Rust"),
    color: (125, 25, 200),
    quit: true,
};

state.process(Message::ChangeColor(100, 222, 50));

println!("{:?}", state.message);     // "Desenvolvedor Rust"
println!("{:?}", state.color);       // (100, 222, 50) ← cor atualizada!
println!("{:?}", state.position.x);  // 1
println!("{:?}", state.position.y);  // 2
println!("{:?}", state.position);    // Point { x: 1, y: 2 }
```
