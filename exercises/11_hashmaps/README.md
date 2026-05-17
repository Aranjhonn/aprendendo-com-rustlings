# Hashmaps

A *hash map* allows you to associate a value with a particular key.
You may also know this by the names [*unordered map* in C++](https://en.cppreference.com/w/cpp/container/unordered_map),
[*dictionary* in Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) or an *associative array* in other languages.

This is the other data structure that we've been talking about before, when
talking about Vecs.

## Further information

- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

---

## Meu Diário de Aprendizado

**`hashmaps1.rs`**
- **O Problema:** Inicializar e preencher um `HashMap`. Tive um erro porque esqueci de retornar a cesta no final da função (o compilador esperava `HashMap` e estava recebendo `()`).
- **O que aprendi:** Como criar um mapa com `HashMap::new()` e adicionar itens com o método `.insert(chave, valor)`.

**`hashmaps2.rs`**
- **O Problema:** Inserir frutas num `HashMap` apenas se elas **ainda não existissem** na cesta, ignorando as que já estavam lá.
- **O que aprendi:** O método `.entry(chave).or_insert(valor)` é extremamente útil. Ele busca a chave e, se estiver vazia, insere o valor padrão que informamos. Se a chave já existir, ele não faz nada.

**`hashmaps3.rs`**
- **O Problema:** Processar resultados de jogos de futebol e acumular os gols marcados e sofridos por cada time em um mapa.
- **O que aprendi:** 
  - O irmão do `or_insert` é o `.or_default()`, que insere um valor "zerado" se a chave não existir (possível graças ao `#[derive(Default)]` na struct `TeamScores`).
  - O retorno de `.entry()` permite alterar os campos de uma struct diretamente com operações como `+=`.
- **Pegadinha Lógica:** Tentei somar os gols sofridos (`goals_conceded`) de um time usando os próprios gols que o time fez! A lógica de futebol é clara: os gols sofridos por um time são os gols *marcados* pelo time adversário! Corrigi cruzando as variáveis.
- **Pegadinha do Parse:** O `.parse::<u8>()` não aceita espaços em branco. Se a string estiver como `" 3"`, ele retorna um erro `ParseIntError { kind: InvalidDigit }`. Precisei limpar os espaços depois das vírgulas.

---

## 🧪 Experimentos no `main()`

No `hashmaps3.rs`, decidi simular um campeonato com resultados reais para testar o código:
```rust
let resultados = "Flamengo,AtléticoPR,3,1\nCorinthians,Palmeiras,1,0\nFlamengo,Corinthians,4,0";

let tabela = build_scores_table(resultados);

for (time, placar) in tabela {
    println!(
        "Time: {} | Fez: {} gols | Sofreu: {} gols",
        time, placar.goals_scored, placar.goals_conceded
    );
}
```
**Resultado:** O HashMap guardou direitinho os dados de cada time, atualizando a mesma entrada quando o Flamengo ou o Corinthians jogaram mais de uma vez!
