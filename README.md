RPG Helper Bot is released under the MIT license. 

This project is primarially for helping me to learn Rust programming, but I hope to have at least a minimally useful product when I'm done.

This is a discord bot intended to assist with using Discord as a forum to play pencil & paper RPGs. Eventually it will have core functionality of rolling dice. I also intend to add the ability to support optional build modules to add features for specific games such as tracking character info and commands to perform skill/attack/damage/etc. rolls for that game (possibly taking into account tracked character info), and to create at least one such module. The evenual goal being to have a program such that anyone can build a bot and run it on their server with their choice of modules, to facilitate the games they want to run.

***Due to discord API changes, all prefix commands will be removed or converted to slash commands in the future***

## Commands
* !ping - Check that bot can process your commands. Bot replies "pong!"
* !r, !roll - Roll dice. Requires a roll specification as a parameter to this command. Roll specification is [number of dice]d[sides](+-[number to add to final roll])(*[number of times to repeat the roll]). Number of sides defaults to one if omitted. Examples of valid roll specifications:
  * 2d4 (roll 2 4-sided dice and return the tota)
  * d6-3 (roll 1 6-sided die, subtract 3 from the roll, and return the total)
  * 3d6*6 (roll 3 6-sided dice, 6 times, and return each total)
  * 2d8+2*3 (roll 2 8-sided dice and add 2 to the roll. Repeat this 3 times and return each total)

## Game specific commands
### Cyberpunk 2020
#### Prefix commands
* !cp_init - Roll initiative. If you have an active tracked character, apply initiative score and combat sense if applicable.
* !cp_skill <skillname> - Make a skill roll. If you have an active tracked character, and specify a skill, the appropriate attribute and skill rank are applied.
* !cp_pick_char <character> - If character is unspecified, it will show a list of your characters which are currently being tracked. If character name (or number from the displayed list) is specified, set that character as your active character.

#### slash commands
* /add <character_name> - Add a tracked character to the database. Not yet fully implemented.
* /init - Equivalent to !cp_init above.
* /pick_char <character> - Equivalent to !cp_pick_char above.
* /skill <skillname> - Equivalent to !cp_skill above.

***Considering converting these to sub-commands of a single game specific command when I start adding support for additional games
eg: /cp2020 add <character> instead of /add <character> ***

## Current status
* Bot can connect to server and listen for commands. 
* Basic dice rolling functionality is implemented.
* Support for controlling settings via config file or command line options.
* Support for game-specific commands for the following games: Cyberpunk 2020
* Support for character tracking for the following games: Cyberpunk 2020 (partial)


## Immediate development goals:
* Remove the prefix commands which have slash command equivalents.
* Look into converting the game-specific slash commands to sub-commands of a game-specific command (i.e. instead of /add something like /cp2020 add)
* Need to implement adding a character to the bot.
* Need to implement editing a tracked character.
