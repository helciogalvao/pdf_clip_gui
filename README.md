# PDF Clip GUI

Uma interface gráfica simples em Rust que permite:

- Selecionar um arquivo PDF
- Informar um número de página
- Copiar a página como imagem PNG diretamente para a área de transferência (via `pdftoppm` e `wl-copy`)

---

## 🔧 Requisitos

Certifique-se de ter os seguintes pacotes instalados:

```bash
sudo apt install poppler-utils wl-clipboard

🚀 Como usar

    Clone o repositório:

git clone git@github.com:SEU_USUARIO/pdf_clip_gui.git
cd pdf_clip_gui

Compile e execute o projeto:

    cargo run

    Na janela que abrir:

        Clique em Selecionar PDF

        Digite o número da página desejada

        Clique em Executar

        A imagem da página será copiada para a área de transferência

💡 Tecnologias utilizadas

    Rust

    eframe / egui

    rfd

    pdftoppm (via poppler-utils)

    wl-copy (via wl-clipboard)

📦 Compilação para produção

Para compilar um executável com otimizações:

cargo build --release

O binário será gerado em target/release/pdf_clip_gui.
📄 Licença

Este projeto é open-source, sob a licença MIT.