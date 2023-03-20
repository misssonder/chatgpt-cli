#  ChatGPT Client
## Install
```shell
cargo install --git https://github.com/misssonder/chatgpt-cli
```
## Usage
Get API key from [https://platform.openai.com/account/api-keys](https://platform.openai.com/account/api-keys)
```shell
ChatGPT command line that support multiple prompts

Usage: chatgpt [OPTIONS] --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>  Api key of the openai support
  -p, --prompt             Customize the prompt
  -m, --model <MODEL>      Set the chatGPT model, default value is 'gpt-3.5-turbo'
  -h, --help               Print help
  -V, --version            Print version

```
### Prompts
#### support prompts
![](./images/prompts.png)
- Default 
- LinuxTerminal 
- Translator
- Interviewer
- JavaScriptConsole
- ExcelSheet
- EnglishPronunciation
- EnglishTeacher
- TravelGuide

you can click [awesome-chatgpt-prompts](https://github.com/f/awesome-chatgpt-prompts) to know more about chatGPT system prompt.