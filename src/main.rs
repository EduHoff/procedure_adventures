fn main() {
    println!("Definição da estrutura do projeto!");

    /*
    Ideias para eu poder me guiar e criar a base do projeto

    1. src/core/ (Definições de Objetos)
        O que vai dentro: O enum Tile, a struct Player e a struct Map.
        Por que: Aqui fica a "alma" do jogo, apenas os dados e as regras básicas, sem saber como eles aparecem na tela.

    2. src/engine/ (Movimentação e Regras)
        O que vai dentro: A lógica que decide se o jogador pode se mover (colisão) e o cálculo das novas coordenadas.
        Por que: Separa o que o jogador é do como ele se comporta.

    3. src/display/ (Impressão na tela)
        O que vai dentro: Funções que recebem o Map e o Player, dão o clear no terminal e imprimem os caracteres.
        Por que: Se um dia você decidir sair do terminal e usar uma biblioteca gráfica, você só precisará mexer nessa pasta.
     */
}
