# 🧠 IA-Vision

**Assistente de Automação de Desktop com IA Local**

OpenVy-Win é um "segundo cérebro" que pode ver sua tela, entender comandos em linguagem natural e executar ações automaticamente no seu computador Windows.

---

## ✨ Funcionalidades

- 🖥️ **Captura de Tela** - Captura a janela ativa e identifica elementos da interface
- 🤖 **IA Local (Ollama)** - Processa comandos usando LLM local (sem enviar dados para nuvem)
- ⚡ **Automação Windows** - Executa cliques e digitação automaticamente
- 💬 **Interface de Chat** - Interface moderna para interagir com a IA
- 🔄 **Auto-inicialização** - Ollama inicia automaticamente com o app

---

## 🏗️ Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    OpenVy-Win (Tauri App)                   │
├─────────────────────────────────────────────────────────────┤
│  Frontend (Next.js 15 + React 19 + TypeScript)              │
│  └── chat-interface.tsx  →  Interface de chat               │
│  └── llm.ts              →  Conexão com Ollama              │
├─────────────────────────────────────────────────────────────┤
│  Backend (Rust)                                              │
│  └── automation.rs  →  UI Automation (Win32 API)            │
│  └── ingestion.rs   →  Captura de tela + OCR                │
│  └── vision.rs      →  Sistema de visão                     │
│  └── main.rs        →  Entry point + auto-start Ollama      │
└─────────────────────────────────────────────────────────────┘
```

---

## 🛠️ Stack Tecnológica

| Componente | Tecnologia |
|------------|------------|
| **Framework Desktop** | Tauri 2.0 |
| **Frontend** | Next.js 15, React 19, TypeScript, TailwindCSS |
| **Backend** | Rust |
| **UI Automation** | Windows Win32 API (IUIAutomation) |
| **Simulação de Input** | Enigo 0.3.0 |
| **LLM Local** | Ollama (Llama3) |
| **OCR** | OCRS |
| **Captura de Tela** | xcap |
| **Database** | SQLx + SQLite |

---

## 📋 Pré-requisitos

Antes de iniciar, certifique-se de ter instalado:

1. **Node.js** (v18+) e **npm**
2. **Rust** e **Cargo** - [rustup.rs](https://rustup.rs)
3. **Visual Studio Build Tools 2022** com:
   - MSVC v143 - VS 2022 C++ x64/x86 build tools
   - Windows 10/11 SDK
4. **Ollama** - [ollama.com/download](https://ollama.com/download)

---

## 🚀 Como Executar

### 1. Clone o repositório
```bash
git clone https://github.com/Charles062/IA-Vision.git
cd IA-Vision
```

### 2. Instale as dependências
```bash
npm install
```

### 3. Baixe o modelo Llama3 (primeira vez)
```bash
ollama pull llama3
```

### 4. Execute em modo desenvolvimento
```bash
npx tauri dev
```

> **Nota:** O Ollama será iniciado automaticamente quando o app abrir.

---

## 💻 Como Usar

1. Abra a aplicação OpenVy-Win
2. Digite um comando na interface de chat, por exemplo:
   - `"Abra o Notepad e digite Olá Mundo"`
   - `"Clique no botão Arquivo"`
   - `"Digite meu nome no campo de texto"`
3. A IA irá:
   - Analisar a tela atual
   - Planejar as ações necessárias
   - Executar os cliques e digitação automaticamente

---

## 📁 Estrutura de Arquivos

```
IA-Vision/
├── app/
│   ├── page.tsx                    # Página principal
│   ├── components/
│   │   └── chat-interface.tsx      # Interface de chat com a IA
│   └── services/
│       └── llm.ts                  # Conexão com Ollama
├── src-tauri/
│   ├── src/
│   │   ├── main.rs                 # Entry point + auto-start Ollama
│   │   ├── automation.rs           # UI Automation (Win32 API)
│   │   ├── ingestion.rs            # Captura de tela + OCR
│   │   └── vision.rs               # Sistema de visão
│   ├── Cargo.toml                  # Dependências Rust
│   ├── tauri.conf.json             # Configuração Tauri
│   └── icons/                      # Ícones do app
├── package.json                    # Dependências Node.js
└── README.md                       # Este arquivo
```

---

## 🔧 Correções e Melhorias Recentes

### v0.1.0 (Janeiro 2026)

- ✅ **Auto-start do Ollama** - O servidor Ollama inicia automaticamente com o modelo llama3
- ✅ **Correção Enigo 0.3.0** - Atualizada API de automação de mouse/teclado
- ✅ **Correção Windows API** - Consertados problemas de tipo com `HWND` e `UIA_CONTROLTYPE_ID`
- ✅ **Geração de Ícones** - Criados ícones válidos para Windows, iOS e Android
- ✅ **Instalação @tauri-apps/api** - Adicionada dependência de API do Tauri para o frontend
- ✅ **Remoção Screenpipe** - Removida dependência de sidecar não disponível
- ✅ **Merge de Branches** - Integração do chat frontend e serviço LLM da branch openvy-win

---

## 🐛 Problemas Conhecidos

- O modelo llama3 pode demorar alguns segundos para carregar na primeira execução
- Requer Windows 10/11 para funcionar (usa APIs específicas do Windows)
- O OCR ainda está em desenvolvimento (placeholder ativo)

---

## 📄 Licença

Este projeto é privado e pertence a Charles062.

---

## 🤝 Contribuição

Para contribuir com o projeto:

1. Crie uma branch a partir de `Main-IA`
2. Faça suas alterações
3. Abra um Pull Request

---

**Desenvolvido com ❤️ usando Tauri, Next.js e Rust**
