# Comfy Demos

This repo currently contains one opensource demo game made for the [1-bit game
jam](https://logloggames.itch.io/bitmob-1-bit-jam). The code remains unchanged,
while the music/SFX were replaced with our own SFX so that we could publish
them with a permissive license (the original assets came from asset packs).

# Disclaimers

We (the authors at LogLog Games) are not in the business of making cleanest
code examples and pretty API abstractions. This game is a product of
implementing software raytracing and a simple game in a few days for a game
jam, while also doing other things. This is not a perfect textbook quality
implementation of raytracing, or the fastest code, or the best architecture
showing how to make the cleanest game in Comfy.

Instead, this is mostly the code as it was written for the game jam, with a
little bit of cleanup (mostly just deleting dead code). Some dead code is left
around for the tinkerers who want to play around with the codebase and maybe
discover some things.

There will not be extensive documentation or support for this codebase. We're
sharing this because we want to show some real code, not because we want this
to be an authoritative example of how to do things.

# Game architecture

**This is not a typical comfy game**. The whole game is a single texture that's
drawn in a single sprite, where each pixel is calculated by raytracing on the
CPU on every frame. What you see as "sprites" (player, enemy, gun) are just
pixels that get copied from the source asset into the target texture one by
one, every frame. They're copied in the right order, which is why things look
the way they are. [This whole process is done by the `blit_at`
function](https://github.com/darthdeus/demos/blob/master/bitmob/src/utils.rs#L6-L52).

The raytracing is also calculated quite simply by the [calculate
lighting](https://github.com/darthdeus/demos/blob/master/bitmob/src/lighting.rs#L3-L130)
function. If you're expecting fancy BVH or other spatial structures prepare
yourself to be deeply disappointed. The raytracer just steps each ray pixel by
pixel and checks for a wall in the grid.

Is this a terrible idea? Maybe. The game runs at a small resolution, so on
desktop it should be "fast enough". If you want to do this in a real game and
ship it to players so that it'll run on a 15 year old potato laptop, you may
want to do some profiling before releasing the game, and maybe utilize some
fancier data structures.

[Comfy now has an experiemental spatial
hash](https://github.com/darthdeus/comfy/blob/master/comfy-core/src/spatial_hash.rs)
which could be used here. We have a different game in the works that uses it
for software raytracing. Note that the spatial hash is considered unstable and
will 100% change in the near future. That being said, if you like it, you
should be able to just take the code and modify it to do whatever you want.
This goes for the whole repo.

# Running the game

To run the game simply run `make` and everything should work.

To build for WASM, run `make wasm`. This requires you to have `trunk`
installed. This will produce a `bitmob/dist` dir with everything needed in it.
You can then run `python3 -m http.server` in the `bitmob/dist` dir and open
`localhost:8000` in your browser, or simply `make serve`.

# Support

All of the code in this repo is meant as a showcase only, and will not be
supported in any way. If you're using it, you're on your own :) Feel free to
ask questions in our Discord, and if there are problems that are related to
Comfy Engine we will 100% address them.

# License

All of the code in this repository is triple licensed under MIT, Apache 2.0,
and The Unlicense. This means you can use it under any of these licenses.

- MIT: https://choosealicense.com/licenses/mit/
- Apache 2.0: https://choosealicense.com/licenses/apache-2.0/
- The Unlicense: https://choosealicense.com/licenses/unlicense/

The reason for this is we really don't care what you do with this code, but
some people might prefer a license they know over a random "public domain"
license, hence MIT/Apache.

All of the assets (images, music, SFX) are licensed under CC0, which means "public domain",
which again means "you can do whatever you want, we don't care, you don't have to give credit".

- https://creativecommons.org/public-domain/cc0/

If you want git clone this repo, compile it, put it on Steam and name it `The
Official Comfiest Comfy Game in Comfy Engine` and sell it for money without
making any changes, go ahead. The above licenses should allow this, and in case
you want clear permission, consider this to be the explicit permission.
