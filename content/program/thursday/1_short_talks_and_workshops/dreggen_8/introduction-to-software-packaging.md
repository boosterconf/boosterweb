---
title: "Introduction to software packaging"
talk_type: "Workshop 3t"
type: talk
authors:
    - Jørgen Kvalsvik

---
Software packaging and distribution is a very simple value proposition, and a surprisingly rich and complex problem underneath. There is a myriad of systems available, and in this workshop will have a closer look at a few common ones. We will go through the stages of building and packaging, and move from a hand-packaged tarball, through dpkg/apt, to the deeply programmable packaging systems guix and nix.
All work and no play makes a dull shop, so it is recommended to bring a computer with some tools installed (see list at the end). All demonstrations will be on Debian, but should translate reasonably well across all unix systems (probably also including WSL). That being said, a computer is not necessary for the workshop, it is fine to show up without one. Exercises will be participant driven.

Leave your docker at home.

Useful software to pre-install:
- A compiler (gcc, clang)
- Coreutils (tar, make, gzip etc.)
- git
- Python
- dpkg-dev (debian, ubuntu) or similar for your distribution
- A working guix and/or nix install [1] [2]

[1] https://guix.gnu.org/manual/en/html_node/Installation.html
[2] https://nixos.org/manual/nix/stable/installation/