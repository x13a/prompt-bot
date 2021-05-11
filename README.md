# tg-prompt

Telegram prompt.

This is a cli utility that can be used to process event which requires user interaction.
For example, it can be used like google prompt login.

It sends message via telegram to your chat with inline keyboard. When you push button, it gets that text, print it 
to stdout and exit. Or you can use "-s" switch to exit success only if first button text is pushed else it will exit 
failure.

## Installation
```sh
$ make
$ make install
```
or
```sh
$ brew tap x13a/tap
$ brew install x13a/tap/tg-prompt
```

## Usage
```text
Usage: tg-prompt [-t <token>] -c <chat-id> -m <message> [-k <keyboard>] [-s]

Telegram prompt.

Options:
  -t, --token       telegram bot token (env: TG_PROMPT_TOKEN)
  -c, --chat-id     chat id
  -m, --message     message
  -k, --keyboard    inline keyboard, use "," for button delimiter, ":" for row
                    delimiter (default: Yes,No)
  -s, --silent      exit success if first button text is pushed else failure
  --help            display usage information
```

## Example

To find chat id:
```sh
$  TG_PROMPT_TOKEN="YOUR_BOT_TOKEN" tg-prompt -c 0 -m ""
```

To get prompt:
```sh
$  TG_PROMPT_TOKEN="YOUR_BOT_TOKEN" tg-prompt -c "CHAT_ID" -m "Hello there?"
```
