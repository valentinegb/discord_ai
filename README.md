# discord_ai
A little project of mine where I experiment with making a sort of Discord AI bot using OpenAI's HTTP API.

## Goal
Alright, I'm setting a goal for this so that I can figure out what exactly I need to get done,
and what I want the end product to be like.

The goal is to have an AI that you can invoke via a slash command from a Discord bot. Something like this:

> **robynn:** ratio
>
> **valentinegb:** \*gasp* how dare you
>
> **valentinegb:** /prompt AI-bert reverse their ratio
>
> **AI-bert:** Ratio.
>
> **robynn:** bruh what the

The AI will only be invoked with that command, it shouldn't respond to anything otherwise.

When a prompt is sent, the last 10 or so messages will also be passed for context.
