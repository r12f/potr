# Potr

![Potr Crate](https://img.shields.io/crates/v/potr.svg)

Potr (Po Translator) is a command line tool for translating [Gettext](https://www.gnu.org/software/gettext/) PO files.

Currently, it supports translation using OpenAI, Azure OpenAI Service, and DeepL.

## Installation

```bash
cargo install potr
```

## Usage

### Translate PO files

To start translating the PO files and update the original file inplace, we can use the `potr` command:

```bash
potr -p <po-file> -e <engine> -t <target-languange> -k <api-key> ...
```

The target languange is defined using the [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) code, e.g. en = English, zh = Chinese, de = German, fr = French, etc.

For examples:

```bash
# Tranlate en.po to English using OpenAI
potr -p en.po -e openai -t en -k <openai-api-key>

# Translate en.po to English using DeepL
potr -p en.po -e deepl -t en -k <deepl-api-key>
```

Translation might take long time, but no worries, we can use `Ctrl + C` to stop processing further messages and the translated messages will be saved to the PO file.

### Message skipping

By default, potr will skip certain messages, such as translated messages and code blocks (message starts with `` ``` ``), we can use certain flags to control this behavior:

- `--pt` or `--process-translated`: Process translated messages
- `--pc` or `--process-code-blocks`: Process code blocks
- `--skip-text`: Skip normal text messages (non-code-blocks)
- `--st` or `--skip-translation`: Skip translation for all messages. This is useful when we only want to update the PO file with the current message to see format changes, etc.
- `-l` or `--limit``: Limit the number of messages to translate. This is useful for testing purpose.

```bash
# Translate en.po to English using OpenAI, process translated messages, skip code blocks, and limit to 3 messages
potr -p en.po -e openai -t en -k <openai-api-key> -l 3 --pt
```

### Environment variables

We can also specify the API key for each translation service using environment variables:

```bash
# DeepL API key
export POTR_API_KEY_DEEPL="..."

# OpenAI API key
export POTR_API_KEY_OPENAI="..."

# Azure OpenAI Service settings
export POTR_API_KEY_AZURE_OPENAI="..."
export POTR_API_BASE_AZURE_OPENAI="..."
export POTR_API_DEPLOYMENT_ID_AZURE_OPENAI="..."
```

Or, in Powershell on Windows:

```powershell
# DeepL API key
$env:POTR_API_KEY_DEEPL="..."

# OpenAI API key
$env:POTR_API_KEY_OPENAI="..."

# Azure OpenAI Service settings
$env:POTR_API_KEY_AZURE_OPENAI="..."
$env:POTR_API_BASE_AZURE_OPENAI="..."
$env:POTR_API_DEPLOYMENT_ID_AZURE_OPENAI="..."
```

### PO file manipulation

Beside translating messages in PO files, Potr also includes 2 tools for manipulating messages in PO files: `Clear` and `Clone`. Using the message skipping flags mentioned above, we can use these tools to clean up the PO files or clone certain messages in the PO files.

```bash
# Remove all current translations (--pc is not specified, so code blocks will be skipped by default)
potr -p en.po -e clear --pt

# Clone all code blocks as it is (process translated messages and code blocks, skip normal text messages)
potr -p en.po -e clone --pt --pc --skip-text
```

## Dev related

### Build

Potr is written in Rust. Building Potr is just like all the other rust projects:

```bash
cargo build
```

### Test

For running unit tests in Potr, we also need to have a valid API key for each translation service. The API keys are fetched from environment variables in the same way as we setup for the `potr` command. Please see "Usage" section above for more details.

Then, we can run the tests:

```bash
cargo test
```

## License
Apache-2.0: <https://www.apache.org/licenses/LICENSE-2.0>