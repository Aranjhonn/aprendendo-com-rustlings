# Move Semantics

These exercises are adapted from [pnkfelix](https://github.com/pnkfelix)'s [Rust Tutorial](https://pnkfelix.github.io/rust-examples-icfp2014/) -- Thank you Felix!!!

## Further information

For this section, the book links are especially important.

- [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Reference and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

## Meu Diário de Aprendizado

**`move_semantics1.rs`**
- **O Problema:** Entender por que não podemos usar uma variável depois de passá-la para uma função.
- **O que aprendi:** O conceito fundamental de **Ownership** (Posse). No Rust, tipos complexos (como `Vec`) não são copiados por padrão; eles são **movidos**. Quando passei o `vec0` para a função `fill_vec`, a posse do dado foi transferida. Tentar usar `vec0` depois disso gera o erro `borrow of moved value`.
- **A Solução:** Para o código compilar, tive que aceitar que `vec0` não existe mais no escopo do `main` após a chamada da função, ou usar o método `.clone()` se eu realmente precisasse de uma cópia (o que tem um custo de performance).

**`move_semantics2.rs`**
- **O Problema:** O teste exigia que tanto `vec0` quanto `vec1` estivessem acessíveis **ao mesmo tempo** após a chamada da função. Mas se o `fill_vec` recebe a posse do vetor (como no exercício 1), o original morre.
- **O que aprendi:** O conceito de **Borrowing** (Empréstimo). Em vez de entregar a posse com `fill_vec(vec0)`, podemos emprestar o dado usando o símbolo `&`: `fill_vec(&vec0)`. A função recebe apenas uma **referência** (um "endereço" para onde o dado está) e o dono original continua vivo e acessível.
- **A Correção:** Mudei a assinatura da função para `fn fill_vec(vec: &Vec<i32>)` e dentro dela criei uma cópia local com `.to_vec()` para poder modificar. Na chamada, passei `&vec0` em vez de `vec0`.

**`move_semantics3.rs`**
- **O Problema:** A função recebia o vetor e precisava modificá-lo (dar `.push()`), mas no exercício 1 isso exigia uma linha extra (`let mut vec = vec;`) dentro do corpo da função.
- **O que aprendi:** O Rust permite declarar a **mutabilidade direto no parâmetro** da função usando `mut` antes do nome: `fn fill_vec(mut vec: Vec<i32>)`. Isso economiza aquela linha interna e deixa o código mais limpo e idiomático.
- **Observação:** A função ainda **toma a posse** do vetor (não usa `&`), então o vetor original morre após ser passado — exatamente como no exercício 1.

**`move_semantics4.rs`**
- **O Problema:** Entender como funcionam as **Referências Mutáveis** (`&mut`) e a regra de que só pode existir uma por vez.
- **O que aprendi:** A diferença entre `&` (referência imutável — só lê) e `&mut` (referência mutável — lê e escreve). O Rust garante que só **uma referência mutável** esteja ativa por vez para evitar conflitos de dados. A referência anterior precisa "morrer" (sair de uso) antes de criar uma nova.
- **Pegadinha:** Inicialmente tentei usar `&vetor` (sem `mut`) e depois chamar `.push()`. O compilador reclamou: *"cannot borrow as mutable, as it is behind a `&` reference"*. Precisei trocar para `&mut vetor`.

**`move_semantics5.rs`**
- **O Problema:** Consolidar tudo: saber quando usar `&` (empréstimo imutável) e quando **não usar** nada (transferir a posse). A função `get_char` apenas lê um caractere — não precisa ser dona da String. Já a função `string_uppercase` transforma o dado internamente — faz sentido ela tomar a posse.
- **O que aprendi:** A decisão de usar `&`, `&mut` ou nenhum dos dois é uma **decisão de design**. Pergunte-se: "essa função precisa ser dona do dado, ou só precisa dar uma olhada?"
- **Experimento provocado:** Tentei imprimir `data` depois de `string_uppercase(data)` de propósito para confirmar que o erro `borrow of moved value` apareceria — e apareceu! Ownership consolidado. 💪

---

## 🧪 Experimentos no `main()`

**`move_semantics1.rs`** — Forçando o erro de Ownership:
```rust
let vec0 = vec![22, 44, 66];
let vec1 = fill_vec(vec0); // vec0 é MOVIDO para aqui
println!("Vetor 1 (novo): {:?}", vec1);
// println!("Vetor 0 (antigo): {:?}", vec0); // ❌ ERRO: value borrowed here after move
```

**`move_semantics2.rs`** — Provando que o Borrowing mantém o original vivo:
```rust
let vetor = vec![0, 2, 4, 6, 8, 10];
let vetor2 = fill_vec(&vetor); // Empréstimo com &
println!("Primeiro vetor: {:?}", vetor);   // ✅ Continua acessível!
println!("Segundo vetor: {:?}", vetor2);   // ✅ Novo vetor com o 88
```

**`move_semantics3.rs`** — Mesmo comportamento do exercício 1 (Move), mas com `mut` direto no parâmetro:
```rust
let vetor1 = vec![1, 3, 5, 7, 9];
let vetor2 = fill_vec(vetor1);   // vetor1 é movido (a função toma posse)
println!("Vetor 1 {:?}", vetor2); // Saída: [1, 3, 5, 7, 9, 88]
// vetor1 não pode mais ser usado aqui!
```

**`move_semantics4.rs`** — Usando referências mutáveis (`&mut`) para modificar sem mover:
```rust
let mut vetor_mut1 = Vec::new();

let vetor_mut2 = &mut vetor_mut1;  // Primeira ref mutável
vetor_mut2.push("João");            // Modifica através da referência

let vetor_mut3 = &mut vetor_mut1;  // Segunda ref mutável (a primeira já "morreu")
vetor_mut3.push("Pedro");

println!("Valor final do vetor 1: {:?}", vetor_mut1); // ["João", "Pedro"]
```

**`move_semantics5.rs`** — Empréstimo vs Move na mesma função `main`:
```rust
let data = "Rust is great!".to_string();

get_char(&data);                          // Empréstimo: data sobrevive
println!("Imprimindo 'data' {}", data);   // ✅ Ainda acessível!

string_uppercase(data);                   // Move: data morre aqui
// println!("{}", data);                  // ❌ ERRO: value borrowed here after move
```
Estes experimentos provaram que o Rust impede o uso de "ponteiros suspensos" (dangling pointers) ou dados que já foram invalidados, garantindo segurança de memória sem um Garbage Collector.
