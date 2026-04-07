# If

`if`, the most basic (but still surprisingly versatile!) type of control flow, is what you'll learn here.

## Further information

- [Control Flow - if expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

---

## Meu Diário de Aprendizado

**`if1.rs`**
- **O Problema:** A função precisava retornar o maior fator numérico entre dois números.
- **A Correção:** Implementamos uma verificação `if a > b { a } else { b }` de forma clássica. A beleza do Rust aqui foi notarmos que não precisamos da palavra `return` em nenhuma das ramificações, aproveitando do Retorno Implícito.

**`if2.rs`**
- **O Problema:** Em Rust, quando usamos blocos `if` como se fossem geradores de valores para a função, todos os caminhos lógicos precisam ser rastreados. Se houver um `if` seguido de um `else if`, o compilador exige estritamente a presença de um `else` de fallback final. Sem ele, existiria uma chance do código fluir pro vazio e estourar um erro se a comida não fosse "morango" nem "batata".
- **A Correção:** Adicionei a devida estrutura de fuga `else { "No thanks!" }`.

**`if3.rs`**
- **O Problema de Sintaxe:** O `if` no Rust é uma **expressão** e por isso ele gera um valor lógico para a máquina, o que nos permitiu injetar ele direto na variável `identifier` com o sinal de `=`. O duro teste aqui é que os braços do `if` retornavam números mas outras ramificações perdidas não retornavam do mesmo tipo.
- **O que aprendi (Sintaxe):** Todos os "braços" (`arms`) de um `if/else if/else` que injetam valores em uma variável precisam, inquestionavelmente, retornar **estritamente o exato mesmo tipo de variável**. 
- **A Correção:** Ajustei os valores retornados pelo bloco animal para serem sempre números consistentes para que a variável `identifier` ganhasse de forma limpa a sua tipagem estática.
- **A Pegadinha (Erro de Lógica):** Ao tentar testar a função no meu próprio `main` passando `animal_habitat("Beach")`, o terminal surpreendentemente resultou em `"Unknown"`. O programa compilou, mas devolveu erro porque eu enviei um *Habitat* em vez do nome do *Animal*. Como `"Beach"` não era nem `"crab"`, nem `"gopher"`, nem `"snake"`, o programa caiu no `else` final (`identifier = 0`). A valiosa lição aqui foi que: "O compilador garante não ter vazamento de memória e conflito de tipos, mas a sua interpretação humana do problema de negócios (testar lógica) continua sendo de sua inteira responsabilidade!".

---

## 💡 Dica Bônus: Experimentando com Códigos Vivos

Até a pasta de "if", nós apenas deixávamos que os pequenos testes automáticos silenciosos avisassem que o exercício foi uma vitória. Daqui de "if" em diante, começamos a **modificar ativamente a função `main()`** (que antes vinha em branco) para testar *na prática* o nosso código:
- Chamar blocos como `let comida = picky_eater("potato");` direto no `main`.
- Imprimir isso em tempo real usando formatações simples como `println!("A comida é {}", comida);`. 

> [!TIP]
> Caso tente rodar só `cargo run` no terminal local, o Rust irá reclamar de "could not determine which binary to run". Como a pasta do rustlings guarda 90 exercícios, para rodar só a nossa brincadeira do `main()`, use a flag binária especificando o exercício: **`cargo run --bin if2`**. Isso ignora os testes invisíveis e exibe a nossa impressão final com sucesso!
