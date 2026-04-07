# Primitive Types

Rust has a couple of basic types that are directly implemented into the
compiler. In this section, we'll go through the most important ones.

## Further information

- [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)

---

## Meu Diário de Aprendizado

**`primitive_types1.rs`**
- **O Problema:** Precisávamos criar uma nova variável booleana que fosse o exato inverso de outra variável.
- **A Correção:** Usei o operador de negação lógica (`!`): `let is_evening = !is_morning;`.

**`primitive_types2.rs`**
- **O Problema:** Compreender o uso do tipo *Caracter* (`char`). Em Rust, ele guarda muito mais que só o alfabeto ou tabelas ASCII tradicionais, pois suporta Unicode por padrão (usa 4 bytes de memória direto na caixa).
- **A Correção:** Configurei a variável armazenando um emoji entre aspas simples para atestar isso: `let your_character = '😊';`. Outro detalhe inegociável é que caracteres no Rust sempre usam obrigatoriamente **aspas simples** (`'X'`), enquanto textos (`String`/`&str`) usam aspas duplas (`"X"`).

**`primitive_types3.rs`**
- **O Problema:** O teste exigia um Array gigantesco contendo pelo menos 100 elementos e escrever de um em um no código-fonte seria loucura.
- **A Correção:** Utilizei a fantástica sintaxe de repetição (Short-hand array initialization). Com o código `let a = [0; 100];`, eu disse implicitamente para o compilador: "povoe estes 100 espaços do array sequencialmente com o valor inteiro '0'".

**`primitive_types4.rs`**
- **O Problema:** O teste precisava extrair uma sub-fatia (um *Slice*) cortada de um Array fixo sem precisar clonar tudo, apenas apontando para a memória original.
- **A Correção:** Usei a notação de Intervalo (*Range*) do Rust baseada em índices: `&a[1..4]`. Vale notar que em Rust o limite inicial do intervalo (`1`) é inclusivo, e o limite final (`4`) é sempre "exclusivo" (lê tudo até a corda arrebentar no item 3).

**`primitive_types5.rs` e `primitive_types6.rs`**
- **O Conceito:** O agrupamento de dados através de Tuplas.
- **O Exercício 5 (Destruturação):** Tinha uma tupla mista `cat = ("Furry McFurson", 3.5)` e precisava desmembrá-la de uma vez só. Resolvido reatribuindo ela a "espelhos" (máscaras de extração de variáveis) no mesmo formato: `let (name, age) = cat;`.
- **O Exercício 6 (Indexação Direta):** Caso eu não queira desempacotar toda a tupla como fizemos no exercício 5, para não encher a memória de lixo com a primeira posição se eu só quero a segunda, o Rust habilita a sintaxe focada: acessamos por ponto seguido do índice exato, ou seja, `let second = numbers.1;`.
