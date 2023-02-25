# Inertia
> Powerful rust command-line task manager.

## Overview
> **Warning** - In progress and early stages of development. I'm not great with Rust either.

Inertia is a smart, fast and simple task manager, named after the tendency to do nothing or to remain unchanged (like procrastination.)
It's supposed to be so powerful you won't even be able to think about your inertia any longer. Heh - see what I did there?

I couldn't find a good task manager, so I'm making one. Basic things to me like the difference between a date you're going to do the task
and a date the task has to be done by are missing in so many task managers. They seem to take the approach of you write down what you do
and then have to think about it every time you read your tasks! You should be able to take the stress away from tasks by writing everything
down once so that you can come back to it knowing exactly what to do.

## Features
- [X] Separate when and deadline dates
- [ ] Show next `n` tasks that you should do
- [ ] Separate areas for tasks
- [ ] Linking tasks together
- [ ] A proper tagging system
- [ ] Git synchronization

## Examples
**Creating a task** - which you will do it today, deadline tomorrow
```sh
inertia add "read emails" -w now -d tomorrow
```
**Deleting a task** - with ID 8
```sh
inertia del 8
```
**Show all tasks**
```sh
inertia show
```
