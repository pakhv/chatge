# Chatge
A simple Web Application written in Rust for chatting with [Ollama LLM](https://ollama.ai/) (Large Language Model). [HTMX](https://htmx.org/) is used for rendering the chat itself.

![Chatge](https://github.com/pakhv/chatge/assets/63700241/cb49c19f-fa4d-4d58-8122-061f9d5c7384)

To run the App you will need:
- Ollama model (more about how to download it see [here](https://ollama.ai/))
- Set couple environment variables
  - `CHATGE_OLLAMA_HOST` - url to Ollama model (for example `http://localhost:11434`)
  - `CHATGE_OLLAMA_MODEL`- the name of the Ollama model (for example `llama2`, list of models you can find [here](https://ollama.ai/library))
