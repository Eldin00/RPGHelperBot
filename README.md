RPG Helper Bot is released under the MIT license. 

This project is primarially for helping me to learn Rust programming, but I hope to have at least a minimally useful product when I'm done. The minimal functional bot was built by referencing the tutorial at:
https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/ 

This is a discord bot intended to assist with using Discord as a forum to play pencil & paper RPGs. Eventually it will have core functionality of rolling dice. I also intend to add the ability to create plugins to add features for specific games such as tracking character info and commands to perform skill/attack/damage/etc. rolls for that game (possibly taking into account tracked character info), and to create at least one such plugin. The evenual goal being to have a bot that anyone can set up and run on their server with their choice of plugins to facilitate the games they want to run.

Current status:
* Bot can connect to server and listen for commands. Only a single test command is currently implemented.

Immediate development goals:
* Implement commands for dice rolling.
* Bring serenity up to the current version, and tokio up to the newest version supported by serenity.