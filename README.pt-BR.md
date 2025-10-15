# Inspector GGUF

[Русская версия](README.ru.md) | [English](README.md) | **Português (Brasil)**

Uma ferramenta poderosa e moderna para inspeção de arquivos GGUF (GPT-Generated Unified Format) com interface gráfica intuitiva e capacidades abrangentes de linha de comando.

## 🚀 Visão Geral

Inspector GGUF é uma ferramenta de nível profissional projetada para analisar e explorar arquivos GGUF usados em machine learning e desenvolvimento de modelos de IA. Construído com Rust e apresentando uma GUI moderna alimentada por egui, fornece insights profundos sobre metadados de modelos, configurações de tokenizadores e detalhes de arquitetura de modelos.

## ✨ Recursos

### Funcionalidade Principal
- 🔍 **Análise Profunda de GGUF** - Extração e exibição abrangente de metadados
- 🖥️ **GUI Moderna** - Interface intuitiva com suporte a arrastar e soltar
- 📊 **Filtragem Avançada** - Capacidades de busca e filtro em tempo real
- 🎨 **Design Adaptativo** - Layout responsivo que escala com o tamanho da tela

### Capacidades de Exportação
- 📄 **Múltiplos Formatos** - Exportar para CSV, YAML, Markdown, HTML e PDF
- 💾 **Processamento em Lote** - Lidar com múltiplos arquivos eficientemente
- 🔧 **Modelos Personalizados** - Opções flexíveis de formatação de exportação

### Suporte a Tokenizadores
- 🧠 **Modelos de Chat** - Visualizar e analisar modelos de chat do tokenizador
- 📝 **Análise de Tokens** - Inspecionar tokens GGML e operações de merge
- 🔍 **Manipulação de Dados Binários** - Codificação Base64 para conteúdo binário grande

### Internacionalização
- 🌍 **Suporte Multi-idioma** - Inglês, Russo, Português (Brasileiro)
- 🔄 **Troca Dinâmica de Idioma** - Mudar idioma sem reiniciar
- 📱 **UI Localizada** - Elementos de interface totalmente traduzidos

### Recursos para Desenvolvedores
- ⚡ **Profiling de Performance** - Integração built-in do profiler puffin
- 🔄 **Auto-atualizações** - Verificação automática de atualizações do GitHub releases
- 🎯 **Tratamento de Erros** - Relatório abrangente de erros e recuperação

## 📦 Instalação

### Dos Releases (Recomendado)
Baixe o release mais recente do [GitHub Releases](https://github.com/FerrisMind/inspector-gguf/releases)

### Do Código Fonte
```bash
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf
cargo build --release
```

### Do Crates.io
```bash
cargo install inspector-gguf
```

## 🎯 Uso

### Interface Gráfica

Iniciar a aplicação GUI:
```bash
inspector-gguf --gui
```

**Recursos da GUI:**
- **Arrastar e Soltar** - Simplesmente arraste arquivos GGUF para a janela
- **Navegador de Arquivos** - Use o botão "Carregar" para procurar arquivos
- **Opções de Exportação** - Múltiplos formatos de exportação disponíveis na barra lateral
- **Configurações** - Preferências de idioma e opções de configuração

### Interface de Linha de Comando

#### Uso Básico
```bash
# Analisar um único arquivo GGUF
inspector-gguf path/to/model.gguf

# Exportar para formato específico
inspector-gguf path/to/model.gguf --output metadata.json
```

#### Opções Avançadas
```bash
# Validar diretório de metadados
inspector-gguf --metadata-dir path/to/yaml/files

# Profiling de performance
inspector-gguf --profile

# Verificar diretório GGUF
inspector-gguf --check-dir path/to/gguf/models
```

## 🏗️ Arquitetura

### Estrutura do Projeto
```
src/
├── gui/                    # Componentes GUI
│   ├── app.rs             # Lógica principal da aplicação
│   ├── theme.rs           # Temas e estilização da UI
│   ├── layout.rs          # Utilitários de layout responsivo
│   ├── export.rs          # Funcionalidade de exportação
│   ├── loader.rs          # Carregamento assíncrono de arquivos
│   ├── updater.rs         # Verificação de atualizações
│   └── panels/            # Painéis da UI
│       ├── sidebar.rs     # Barra lateral esquerda com ações
│       ├── content.rs     # Exibição principal do conteúdo
│       └── dialogs.rs     # Diálogos modais
├── localization/          # Internacionalização
│   ├── manager.rs         # Gerenciamento de localização
│   ├── loader.rs          # Carregamento de traduções
│   ├── detector.rs        # Detecção de locale do sistema
│   └── language.rs        # Definições de idiomas
├── format.rs              # Manipulação do formato GGUF
├── lib.rs                 # Exportações da biblioteca
└── main.rs                # Ponto de entrada da aplicação
```

## 🔧 Configuração

### Configurações de Idioma
A aplicação detecta automaticamente o idioma do seu sistema. Idiomas suportados:
- **Inglês** (en) - Padrão
- **Russo** (ru) - Русский
- **Português (Brasileiro)** (pt-BR) - Português (Brasil)

### Ajuste de Performance
Para performance ótima com modelos grandes:
```bash
# Habilitar modo de profiling
inspector-gguf --profile

# Acessar profiler em http://127.0.0.1:8585
```

## 🧪 Testes

Executar a suíte de testes abrangente:
```bash
# Executar todos os testes
cargo test

# Executar com cobertura
cargo test --all-features

# Executar módulos específicos de teste
cargo test gui::export::tests
cargo test localization::tests
```

## 🤝 Contribuindo

Damos as boas-vindas a contribuições! Por favor, veja nosso [Guia de Contribuição](CONTRIBUTING.md) para detalhes.

### Configuração de Desenvolvimento
```bash
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf
cargo build
cargo test
```

### Adicionando Novos Idiomas
1. Criar arquivo de tradução em `translations/{language_code}.json`
2. Adicionar definição de idioma em `src/localization/language.rs`
3. Atualizar detecção de idioma em `src/localization/detector.rs`
4. Testar com `cargo test localization::tests`

## 📋 Requisitos do Sistema

- **Rust** 1.70 ou mais recente
- **Sistemas Operacionais** Windows, macOS, Linux
- **Memória** 512MB RAM mínimo (2GB+ recomendado para modelos grandes)
- **Armazenamento** 50MB para aplicação, espaço adicional para arquivos de modelo

## 🐛 Solução de Problemas

### Problemas Comuns

**Aplicação não inicia:**
- Certifique-se de que o toolchain Rust está instalado corretamente
- Verifique os requisitos do sistema
- Verifique as permissões de arquivo

**Arquivos grandes carregam lentamente:**
- Habilite o modo de profiling para identificar gargalos
- Certifique-se de ter memória suficiente do sistema
- Considere usar armazenamento SSD para melhor performance de I/O

**Falhas de exportação:**
- Verifique permissões de escrita no diretório de destino
- Certifique-se de ter espaço suficiente em disco
- Verifique a validade do caminho do arquivo

## 📚 Documentação

### Suíte Completa de Documentação
- **[📖 Guia do Usuário](docs/USER_GUIDE.md)** - Instruções abrangentes de uso
- **[❓ FAQ](docs/FAQ.md)** - Perguntas frequentes e solução de problemas
- **[🏗️ Arquitetura](docs/ARCHITECTURE.md)** - Arquitetura técnica e design
- **[📋 Documentação da API](docs/API.md)** - Uso da biblioteca e integração
- **[🚀 Guia de Deploy](docs/DEPLOYMENT.md)** - Construção e deployment
- **[🤝 Contribuindo](CONTRIBUTING.md)** - Como contribuir para o projeto

### Links Rápidos
- **[📥 Baixar Último Release](https://github.com/FerrisMind/inspector-gguf/releases/latest)**
- **[🔄 Changelog](CHANGELOG.md)** - Histórico de versões e mudanças
- **[📜 Licença](LICENSE)** - Detalhes da licença MIT
- **[🤝 Código de Conduta](CODE_OF_CONDUCT.md)** - Diretrizes da comunidade

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 🙏 Agradecimentos

- **Candle** - Framework ML baseado em Rust para suporte GGUF
- **egui** - Framework GUI de modo imediato
- **Comunidade** - Contribuidores e usuários que tornam este projeto melhor

## 📞 Suporte

- **[❓ FAQ](docs/FAQ.md)** - Respostas rápidas para perguntas comuns
- **[🐛 Issues](https://github.com/FerrisMind/inspector-gguf/issues)** - Relatórios de bugs e solicitações de recursos
- **[💬 Discussões](https://github.com/FerrisMind/inspector-gguf/discussions)** - Suporte da comunidade e ideias
- **[📧 Email](mailto:contact@ferrismind.com)** - Contato direto para questões de segurança

---

**Feito com ❤️ pela equipe [FerrisMind](https://github.com/FerrisMind)**