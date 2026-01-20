# 🤖 OpenVy-Win (IA-Vision)

> **Agente de Automação de Interface com IA Local** - Controle seu computador usando linguagem natural!

OpenVy-Win é uma aplicação desktop que permite automatizar tarefas no computador através de comandos em linguagem natural. A aplicação usa IA local (Ollama + Qwen 2.5-Coder) para interpretar seus comandos e executar ações na interface gráfica.

---

## 🎯 O que é?

OpenVy-Win é um **agente de automação visual** que:

1. **Captura a tela** do seu computador
2. **Analisa os elementos da interface** (botões, campos de texto, menus)
3. **Interpreta comandos em linguagem natural** usando IA local
4. **Executa ações automaticamente** (cliques, digitação)

### Exemplo de Uso
```
Você: "Abra o Notepad e digite Hello World"

IA planeja:
→ Clicar no botão Notepad
→ Digitar "Hello World"
```

---

## 🏗️ Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    OpenVy-Win (Tauri)                       │
├─────────────────────────────────────────────────────────────┤
│  Frontend (Next.js + React)                                 │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ chat-interface.tsx  │ Interface de chat com usuário    ││
│  │ llm.ts              │ Comunicação com Ollama API       ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│  Backend (Rust)                                             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ main.rs             │ Entry point + Tauri commands     ││
│  │ ingestion.rs        │ Captura de tela + OCR            ││
│  │ automation.rs       │ Execução de ações (mouse/teclado)││
│  │ accessibility/      │ Árvore de elementos UI           ││
│  │   ├── win.rs        │ Windows UI Automation            ││
│  │   └── linux.rs      │ Linux AT-SPI (em desenvolvimento)││
│  │ vision.rs           │ Integração ScreenPipe            ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│  LLM Local                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ Ollama + Qwen 2.5   │ Modelo de IA local               ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

---

## 🔧 Como Funciona

### 1. Captura de Contexto (`ingestion.rs`)
- Captura screenshots da tela a cada 5 segundos
- Identifica a janela ativa (nome do app, título)
- Realiza OCR para extrair texto da tela

### 2. Árvore de Acessibilidade (`accessibility/`)
- **Windows**: Usa UI Automation para obter elementos (botões, campos, etc.)
- **Linux**: Usa AT-SPI (em desenvolvimento)
- Retorna: nome, tipo, coordenadas e estado de cada elemento

### 3. Planejamento com LLM (`llm.ts`)
- Envia o comando do usuário + elementos visíveis para o Ollama
- O LLM retorna um plano de ações em JSON:
```json
[
  {"action": "click", "x": 100, "y": 200, "description": "Clicar no menu"},
  {"action": "type", "text": "Hello", "description": "Digitar texto"}
]
```

### 4. Execução (`automation.rs`)
- **Windows**: Usa `enigo` para controlar mouse/teclado
- **Linux**: Mock implementado (aguardando implementação completa)

---

## 📋 Requisitos

### Sistema
- **OS**: Windows 10/11 (suporte completo) ou Linux (parcial)
- **RAM**: 8GB+ (recomendado 16GB para LLM)
- **GPU**: Opcional, mas acelera o LLM

### Dependências
- [Node.js 18+](https://nodejs.org/)
- [Rust](https://rustup.rs/)
- [Ollama](https://ollama.com/)
- Tesseract OCR (Necessário para leitura de tela)

### Linux (dependências adicionais)
```bash
sudo apt install pkg-config libssl-dev libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev
```

---

## 🚀 Instalação

### 1. Clone o repositório
```bash
git clone https://github.com/Charles062/IA-Vision.git
cd IA-Vision
```

### 2. Instale as dependências
```bash
npm install
```

### 3. Instale o Ollama e o modelo
```bash
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen2.5-coder:1.5b
```

### 4. Execute a aplicação
```bash
npx tauri dev
```

---

## 🛠️ Solução de Problemas

### Erro: "Tesseract not found"
- **Causa**: O binário do Tesseract não está no PATH do sistema.
- **Solução**: O sistema desativará o OCR automaticamente e continuará funcionando (sem leitura de texto). Para ativar, instale o Tesseract e adicione ao PATH.

### Erro: "LLM Response is not an array"
- **Causa**: O modelo de IA retornou uma resposta fora do padrão (ex: objeto único).
- **Correção**: Já aplicada na versão atual. O sistema agora trata automaticamente respostas formatadas incorretamente pelo modelo `qwen2.5-coder`.

### O mouse clica no lugar errado
- **Causa**: Escala de DPI do Windows ou erro de interpretação de coordenadas.
- **Correção**: Certifique-se de que a escala do Windows está em 100% ou use a versão mais recente que corrige o mapeamento de `bounding_box`.

---

## ⚠️ O que está Faltando

### 🔴 Crítico (Necessário para funcionar)

| Componente | Status | Descrição |
|------------|--------|-----------|
| **Linux Accessibility** | ❌ Não implementado | `accessibility/linux.rs` retorna árvore vazia. Precisa implementar AT-SPI 0.19 |
| **Linux Automation** | ❌ Mock | `automation.rs` no Linux apenas imprime no console, não executa ações reais |
| **Screenshot Linux** | ⚠️ Instável | `xcap` falha em alguns ambientes Linux (Wayland, permissões) |
| **OCR Real** | ✅ | Implementado com `rusty-tesseract` em `ingestion.rs` |

### 🟡 Importante (Melhorias significativas)

| Componente | Status | Descrição |
|------------|--------|-----------|
| **Persistência de Contexto** | ❌ Não implementado | Falta SQLite/Vector DB para salvar histórico de capturas |
| **ScreenPipe Integration** | ⚠️ Stub | `vision.rs` tem código de integração mas não está funcional |
| **Tratamento de Erros** | ⚠️ Básico | Erros do LLM/Ollama não são bem tratados na UI |
| **Suporte a macOS** | ❌ Não implementado | Apenas fallback vazio |

### 🟢 Melhorias Futuras

| Feature | Descrição |
|---------|-----------|
| **Multimodal (Visão)** | Usar modelos como LLaVA para "ver" a tela |
| **Múltiplos Monitores** | Suporte a setup multi-monitor |
| **Gravação de Macros** | Gravar e reproduzir sequências de ações |
| **Plugins** | Sistema de extensões para apps específicos |
| **Voice Input** | Comandos por voz |

---

## 📁 Estrutura do Projeto

```
IA-Vision/
├── app/                          # Frontend Next.js
│   ├── components/
│   │   └── chat-interface.tsx    # Interface de chat
│   ├── services/
│   │   └── llm.ts                # Cliente Ollama
│   ├── globals.css               # Estilos globais
│   ├── layout.tsx                # Layout principal
│   └── page.tsx                  # Página inicial
├── src-tauri/                    # Backend Rust
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── automation.rs         # Controle mouse/teclado
│   │   ├── ingestion.rs          # Captura de tela + OCR
│   │   ├── vision.rs             # ScreenPipe integration
│   │   └── accessibility/
│   │       ├── mod.rs            # Module exports
│   │       ├── win.rs            # Windows UI Automation
│   │       └── linux.rs          # Linux AT-SPI
│   ├── Cargo.toml                # Dependências Rust
│   └── tauri.conf.json           # Configuração Tauri
├── package.json                  # Dependências Node.js
└── README.md                     # Este arquivo
```

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Áreas prioritárias:

1. **Implementar Linux AT-SPI** em `accessibility/linux.rs`
2. **Implementar automação Linux** com `enigo` ou `xdotool`
3. **Integrar OCR real** com `ocrs` ou `tesseract`
4. **Melhorar tratamento de erros** na UI

---

## 📝 Licença

MIT License - Veja [LICENSE](LICENSE) para detalhes.

---

## 🔗 Links

- **Repositório**: https://github.com/Charles062/IA-Vision
- **Ollama**: https://ollama.com/
- **Tauri**: https://tauri.app/
