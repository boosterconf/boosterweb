---
title: "Zero to Nix: Build Your Own Reproducible Environments"
talk_type: "Workshop 1,5h"
type: talk
starts_at: 2027-03-17T13:35:00Z
ends_at: 2027-03-17T15:05:00Z
weight: 1
authors:
    - Leonard Sheng Sheng Lee

---
"It works on my machine." Famous last words.

You clone a repository, follow the README, install the dependencies, and discover that your compiler is too new, your Python is too old, or some library has mysteriously disappeared. Meanwhile, your colleague insists everything works perfectly on their laptop.

Containers help, but do you really need to build a Docker image just to get a consistent development environment?

Enter Nix, a package manager that takes a different approach to managing software and dependencies. Instead of relying on whatever happens to be installed on your machine, Nix lets you declare exactly what your project needs and reproduce that environment elsewhere.

In this hands-on workshop, we'll start with a deliberately broken development environment and gradually fix it using Nix. You'll learn how the Nix store works, create development shells, write your first Nix expressions, and use Nix Flakes to define reproducible environments that can be shared across machines and CI pipelines.

We'll also explore what happens when dependencies change, how Nix handles multiple versions of the same tool, and why reproducibility involves more than pinning a version number.

By the end, you'll have a working project with a Nix Flake that defines its development environment. More importantly, you'll understand what Nix actually does, rather than just copying mysterious configuration snippets from the internet.

No previous Nix experience required. Bring your laptop and a willingness to break things.
