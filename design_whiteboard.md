U+1F407

goal: simple but fun and interesting gba game for game jam
timeframe: contest ends september 13 2026 @ 4:59 PM
https://itch.io/jam/gbajam26

idea: morse code game

setting:

blinking lights and shutters surrounding a panopticon
       |---------|
       |         |
mirror |         |bed
/sink  |         |
       |____x____|
          door to hellish
          glitch world

business-like disinterested robot gives you orders and tells you your job
"Of course, you're free to leave at any point"
but leaving doesn't get you anywhere. it's loud and visually :distressing (no flashing, but also not comfy.) when you turn around, the door is always right behind you.
(maybe there's a little tree)

r button pulls up a morse code translator you can tap sequences into.
l button pulls up letter guide that shows you what each letter sounds/looks like
morse in sound as well as flashing lights

start over- actual morse code for start over.

you can lay down in your bed at any point and look at the flashing lights across the panopticon.
you're a cog in a big "AI" machine, severance style.

gameplay loop:
start with one letter at a time.

you start out with sequences of 2 letters you need to decode
you use a cipher with a maze-like structure to determine your output, which you then tap out in morse code.
you get points for every correct answer, extra points for speed. you level up when your average speed gets above x words per minute, then you start getting 3 letters at a time.
(spaced repitition for letter selection)
you can find easter eggs and secrets by getting good at reading the "stars" above you. There are people saying things, giving riddles, etc.
the person to your right will eventually start talking to you through the wall, also in morse code.

stretch goal: tone customizer- waveform, pitch and maybe filters depending on what the GBA has available.
customizable light color too (you and the computer giving you the puzzles.
thinking about accesibility - not sure if the flashing lights will be a problem- give an option to disable it for the puzzles and go by sound only.

Most important thing to have: a morse code engine that will run on the GBA.

code structure estimate:

menu (new game, continue)
                    |
                    V
                  2 save slots
          |         |
          V         V
      tutorial:
      R: tap translator
      L: listen translator
      inputs and outputs
      
          room N (puzzle terminal, window)
              stationed at terminal
          room W (mirror)
          room E (bed)
              sleeping
              scratching
          room S (door)
              out of the door


morse code engine design requirements:
play sounds + display sprites in time to '._/' notation
interpret user input as morse code letters (factoring in human timing error and different tempos)

easy, right? (WRONG, probably. No- you know what? This is going to be easy. easy and fun.)

components of a morse code signal:
"dot" - 1 unit of time
"dash" - 3 units of time
quark* separator - 1 unit of time
letter separator (LS) - 3 units of time
word separator (WS) - 7 units of time
    *a quark is a dot or dash - a component of a letter.

step 1 - record "message"
step 2 - do the math to see what the tempo should be



compact representation of morse code looks like what?
for storing just morse code
26 letters, "." " " "?"
override the debug implementation to represent the compressed morse format as a string

todo: morse code parser - takes a string, gives back a ._/ representation of the morse code

morse encoding:
dot:    00
dash:   01
ls:     10
ws:     11
I think any morse string can be encoded in just those sequences.
an extra dot worth of silence will be added after every dot or dash

the reference I'm using for morse translation.
https://morsecode.world/international/morse.html

"bunny":   _.../.._/_./_./_.__//
encoding:   (b) 01000000 10
            (u) 000010 10
            (n) 0100 10
            (n) 0100 10
            (y) 01000101 11


timing:     (b) 1110 10 10 10
            (ls) 00
(1=sound)   (u) 10 10 1110
            (ls) 00
(0=silence) (n) 1110 10
            (ls) 00
            (n) 1110 10
            (ls) 00
            (y) 1110 10 1110 1110
            (ws) 000000

