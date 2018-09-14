# nevenoe-bot

Discord bot that provides access to breton language tools

## Usage

`!help`
lists all available commands

`!termofis [term]`
directly access the termofis dictionnary and print out result in markdown. fr=>br

`!glosbe [term]`
give result from glosbe dictionnary (fr => br)

`!difazi [sentence]`
corrects the breton sentence from languagetool API. This can also be
triggered by using the reaction emoji "bot_difazian" on a message.

`!troer [sentence]`
Translates the breton sentences using the termofis translator. This can also be
triggered by using the reaction emoji "bot_trein" on a message.

## Installation

run
```shell
DISCORD_TOKEN=[token] cargo run
```
in the root directory
