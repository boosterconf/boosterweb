---
title: "Your coding agent is a while loop: now build one!"
talk_type: "Workshop 3h"
type: talk
starts_at: 2027-03-18T09:00:00Z
ends_at: 2027-03-18T12:20:00Z
weight: 1
authors:
    - Bruno Bossola

---
You use a coding agent every day, but you have never opened one. So you guess what is inside: a planner, an orchestrator, a state machine, something clever.

There is nothing clever in there. There is a loop, and it is about fourteen lines long.

In this workshop you write those fourteen lines yourself, in plain Java (or the language of your choice), in an empty file. Then you spend the rest of the session finding out that the loop was the easy part.

You will add a tool that reads files, and watch a chatbot become an agent. You will add a tool that writes files, ask your agent for a new HTTP endpoint, and curl the endpoint it built. And then you will break it, on purpose, over and over:
- the model answers a question about a file it never opened, and invents the routes.
- the model prints its tool call as prose, and your agent dies with a ClassCastException.
- the model replaces the wrong line, because your text match found two of them.
- the model calls one more tool, then one more, and never stops.
- the model asks to read /etc/passwd, and your agent is running as you.

You fix each one, and by the end you have a small coding agent that works, and roughly a hundred lines of code, of which fourteen are the loop and the rest is armour.

Everything runs on your laptop against a local model through Ollama. No API key, no account, no bill, and nothing breaks when the conference wifi does. You can bring your Anthropic key though! There is no agent framework, no web framework and no complex build. A few files, one command, and, if you choose Java, a repository of git tags so you can jump forward if you fall behind.

You leave with a working agent you can point at your own repository on the train home, and with a mental model that will change how you read every agent transcript you see after this.

After the workshop you will be able to:
- write the agent loop from memory, and articulate what each of the four core types hold
- declare a tool, and explain why the description functions directly as the program
- demonstrate that no implicit planner exists by modifying a tool description and observing the resulting behavior shift
- identify the four standard failure modes from a transcript: missing tool calls, prose-embedded tool calls, ambiguous edits, and infinite loops
- bound every value received from the model, and explain why the model itself cannot enforce these bounds.
- craft effective tool error messages that the model can interpret and act upon autonomously
- establish a trust boundary within a single method and call site, and defend that architectural decision
- analyze a token trace to accurately predict the cost, latency, and context growth of an agent turn
