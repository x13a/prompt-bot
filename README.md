# bot-prompt

Bot prompt.

This is a cli utility that can be used to process events that requires user interaction.
For example, it can be used like google prompt login.

It sends message via `telegram | discord` to your chat with inline keyboard. When you push button, it gets that text, 
print it to stdout and exit. Or you can use `-s` option to exit success only if first button text is pushed else it 
will exit failure.

## Installation
```sh
$ make
$ make install
```
or
```sh
$ brew tap x13a/tap
$ brew install x13a/tap/bot-prompt
```

## Usage
```text
Usage: bot-prompt [-t <token>] -c <chat-id> -m <message> [-k <keyboard>] [-s] <command> [<args>]

Bot prompt.

Options:
  -t, --token       bot token (env: BOT_PROMPT_TOKEN)
  -c, --chat-id     chat id
  -m, --message     message
  -k, --keyboard    inline keyboard, use "," for button delimiter, ":" for row
                    delimiter (default: Yes,No)
  -s, --silent      exit success if first button text is pushed else failure
  --help            display usage information

Commands:
  tg                use telegram
  discord           use discord
```

## Example

To find chat id on telegram:
```sh
$  BOT_PROMPT_TOKEN="YOUR_BOT_TOKEN" bot-prompt -c 0 -m "" tg
```

To get prompt on discord:
```sh
$  BOT_PROMPT_TOKEN="YOUR_BOT_TOKEN" bot-prompt -c <CHAT_ID> -m "Hello there?" discord -a <APP_ID>
```
