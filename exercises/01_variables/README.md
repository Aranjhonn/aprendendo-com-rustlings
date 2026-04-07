# Variables

In Rust, variables are immutable by default.
When a variable is immutable, once a value is bound to a name, you can't change that value.
You can make them mutable by adding `mut` in front of the variable name.

## Further information

- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

## Meu Diário de Aprendizado

**`variables1.rs`**
- **O Problema:** A variável foi declarada sem a palavra-chave que diz ao compilador para criar uma variável de escopo local.
- **A Correção:** Uso da palavra-chave `let` para declarar `let x = 5;`.

**`variables2.rs` e `variables3.rs`**
- **O Problema:** Tentativa de uso de variáveis criadas, mas que ainda não possuíam valor inicial (não foram instanciadas). O Rust proíbe o uso de variáveis nulas ou vazias na leitura!
- **A Correção:** Atribuição do valor inicial usando `= 10`. Note que o Rust infere o tipo na maioria das vezes, mas também podemos declarar expressamente (`let x: i32 = 10;`).

**`variables4.rs`**
- **O Problema:** Em Rust, toda variável criada com `let` no formato padrão é construída como sendo **imutável**. Não podíamos reatribuir o valor de `x` nas linhas seguintes.
- **A Correção:** Transformamos ela numa variável mutável adicionando a palavra mágica *mut*: `let mut x = 3;`.

**`variables5.rs`**
- **O Problema:** Tínhamos uma string "T-H-R-E-E" e queríamos redefini-la como um número inteiro para cálculos. No Rust, você não altera o tipo de uma variável usando reatribuição depois do seu nascimento.
- **O que aprendi:** O conceito de **Shadowing** (Sombreamento). Você pode declarar um novo `let` usando *o exato mesmo nome de uma variável que já existe*. O segundo `let` ofusca/assassina a primeira variável daquele escopo, sendo excelente para transformações de tipos (`String` indo para `Number`)!

**`variables6.rs`**
- **O Problema:** Ao criar constantes (`const`), não informamos o tipo de dado. Diferente do let, o compilador do Rust sempre exige que as constantes tenham o tipo declarado estritamente; ele se recusa a inferir os tipos ali.
- **A Correção:** Adicionado o tipo na declaração logo após o nome: `const NUMBER: i32 = 3;`.
- **Dica de Estilo:** Em Rust, é uma convenção obrigatória (o compilador avisa se não for seguida) que constantes (`const`) devem sempre ser nomeadas utilizando **LETRAS MAIÚSCULAS** (SCREAMING_SNAKE_CASE).
