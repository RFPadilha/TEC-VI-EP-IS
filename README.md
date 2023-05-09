# TEC-VI-EP-IS
Repositório utilizado para enviar o trabalho final das implementações do NPB 3.4.2

## EP
- Cargo.lock mantém informações sobre as dependências das "crates" que rust usa para compilar o programa
- Cargo.toml contém metadados de compilação.
### src
- auxfunctions.rs : arquivo que contém as funções auxiliares "randlc", "vranlc" e "verify", que são utilizadas para gerar números aleatórios e verificar a validade do programa.
- ep_data.rs : arquivo restante das primeiras tentativas de implementação, contém algumas especificações de dados que não são necessárias para a implementação atual. Consta nesse git pela possibilidade de aprimoramento, talvez ainda possa ser útil.
- main.rs : arquivo principal onde são feitas as chamadas de funções auxiliares, implementando o benchmark em si.


## IS
- Cargo.lock mantém informações sobre as dependências das "crates" que rust usa para compilar o programa
- Cargo.toml contém metadados de compilação.
### src
- main.rs : arquivo contendo a implementação do benchmark IS em rust, único arquivo necessário.

## Executando
Caso não possua a instalação de "cargo", instalar com os seguintes comandos: "sudo apt-get install cargo"
- Para ambas as implementações:
1. Abra o terminal no diretório do nome do benchmark desejado
2. Compile com "cargo build"
3. Execute com "cargo run " + "classe do problema"(S, W, A, B ...)
