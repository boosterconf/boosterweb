## The Booster conference website

The content of this site is written in Markdown. Here's a handy markdown [cheatsheet](https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet). 

The markdown files are translated to HTML using a program called [Hugo](https://gohugo.io/), and hosted on Heroku. Anything pushed to main will end up on the website within half a minute or so. 

The easiest way to contribute to the site is to, when viewing this page on github.com/boosterconf/boosterweb, pressing the '.' key. This will open an editor
in your browser, where you can do your changes. Ctrl-S and most other shortcuts works as you would expect. When you have completed your change, commit the
changes to git, using the graph-like icon in the side-bar. [Here's a small demonstration of how it works.](Contributing2.gif)

## Adding conference organizers to "about us"

To add conference organizers to the about-us page add a photo under `content/info/about/who-are-we` and metadata about the person in `content/info/about/index.md` resources.

## Adding conference partners

To add conference partners, add the logo under `content/partners/logos` and metadata about the partner under `content/partners/index.md`. If only the logo is added the partner will still appear in the footer but with incorrect linking.
