use std::env;

use argh::FromArgs;

use tg_prompt::{find_chat_id, get_prompt, Result, BUTTON_SEP, ROW_SEP};

const ENV_TOKEN: &'static str = "TG_PROMPT_TOKEN";

#[derive(FromArgs)]
/// Telegram prompt.
struct Opts {
    #[argh(option, short = 't')]
    /// telegram bot token (env: TG_PROMPT_TOKEN)
    token: Option<String>,

    #[argh(option, short = 'c')]
    /// chat id
    chat_id: i64,

    #[argh(option, short = 'm')]
    /// message
    message: String,

    #[argh(option, short = 'k', default = "String::from(\"Yes,No\")")]
    /// inline keyboard, use "," for button delimiter, ":" for row delimiter (default: Yes,No)
    keyboard: String,

    #[argh(switch, short = 's')]
    /// exit success if first button text is pushed else failure
    silent: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = argh::from_env();
    let token = match opts.token {
        Some(s) => s,
        None => {
            let s = env::var(ENV_TOKEN).expect("token not found");
            env::remove_var(ENV_TOKEN);
            s
        }
    };
    if opts.chat_id == 0 {
        find_chat_id(&token).await?;
    }
    let answer = get_prompt(&token, opts.chat_id, &opts.message, &opts.keyboard).await?;
    if opts.silent {
        if answer
            != opts
                .keyboard
                .split(ROW_SEP)
                .map(|s| s.split(BUTTON_SEP))
                .flatten()
                .next()
                .unwrap_or_default()
        {
            return Err(answer.into());
        }
    } else {
        println!("{}", answer);
    }
    Ok(())
}
