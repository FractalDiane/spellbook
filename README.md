# Introduction

Welcome, aspiring witch! If you're reading this, you must have a knack for magic, and you must be interested in the world of spellbook authoring and publishing. Consequently, it appears you're also interested in strange programming languages that make no sense from a practical standpoint. That's why **SPELLBOOK** is the language for you!

Spellbook is an esoteric programming language based around the magical shenanigans of a financially-challenged witch writing a book of spells. You have some useful, if exasperatingly limited, tools at your disposal, alongside a collection of spells to help you get the job done and publish your spellbook. [WRITE MORE HERE]

# Syntax

Spellbook is a language of magic and its associated words, so it's almost exclusively written in plain English. No pesky operators to worry about here!

## Comments
A fundamental principle of magic is that it only works if you say the words of your spells as inaudibly as possible. Shouting will accomplish nothing but making a fool of yourself. Therefore, shout in all caps if you want to add comments.

```
HELLO THERE THIS IS A COMMENT!
```

# The Spellbook

The most important part of the language is your spellbook you have on your desk, which you'll be writing in to begin executing your desired actions.

## Chapters

Due to budget constraints, you have a *very* small spellbook to write in, with only four different single-page chapters. Not only that, but each chapter can only have three entries in it. The chapters available to you are as follows:

| Chapter Title | Contents |
| ------------- | -------- |
| Presages | Boolean values, `true` or `false`. |
| Hexes | Integer numbers. |
| Illusions | Floating-point numbers. |
| Incantations | Character strings. |

## Turning to and writing in chapter pages

Before you write anything, you'll need to make sure you're turned to the right chapter, with the `turn to chapter` statement. Then you can use `write`...`under` to write something down in the chapter, under whatever heading name you'd like. Remember that each chapter can only have three entries!

```
turn to chapter Illusions
write 2.5 under my_float
write 5.0 under my_float_2
write 7.5 under my_float_3
write 10.0 under my_float_4   ERROR! CAN ONLY PUT THREE ENTRIES ON A PAGE
```

Note that entries are always written into a page in order, filling the first empty slot. For example, if a new entry is written in a page with only its third entry filled, it will be written to the first entry.

If you try to write an entry of a certain type in the wrong page, the spellbook will automagically and implicitmagically convert it for you.

```
turn to chapter Presages
write 2.5 under my_float
THIS WRITES A VALUE OF TRUE

turn to chapter Hexes
write true under my_bool
THIS WRITES A VALUE OF 1

turn to chapter Illusions
write "Hello" under my_string    ERROR! CAN'T CONVERT THAT
```

## Tearing out chapters

You can only have three entries per chapter, and you're writing with a pen like a good author, so there's no erasing those entries once you write them. So, how do you get rid of them? Tear the entire chapter out with `tear out chapter`! The magical spellbook will generate a new blank one for you right away.

Of course, where are you going to put those chapters you tear out? The first option is the simplest: If you're so dissatisfied with your work that you want it gone forever, you can throw it in the trash like the worthless garbage that it is.

```
tear out chapter and throw it in the trash
```

## Publishing

Publishing with `publish spellbook` allows you to share your immaculately-written spellbook with the world. This means putting your spellbook into print, of course.

```
turn to page Incantations
write "Hello, world!" under hello
publish spellbook
```

If the console isn't your idea of a target audience, you can also publish your spellbook to a file.

```
publish spellbook to "hello.txt"
```

However, something very important to keep in mind is that you can't just publish half of a spellbook. That goes against all sorts of publishing conventions. So, when you publish your spellbook, you will publish *everything* currently in it.

```
turn to chapter Incantations
write "Hello, world!" under hello
turn to chapter Hexes
write 52 under some_number
publish spellbook
THIS PRINTS "HELLO, WORLD!" AND THEN 52
```

Something else important to note is that published books need to be as perfectly wrapped up as possible, so by default your published spellbook will be terminated with a nice, fancy "THE END."

If you want to end off your spellbook with something else, `sign acknowledgements page with` whatever string you'd like.
```
STANDARD HELLO WORLD PROGRAM

turn to chapter Incantations
write "Hello, world!" under hello
sign acknowledgements page with "\n"
publish spellbook
```

You can also sign individual chapters with whatever closing you'd like. By default, they're signed with a newline.

```
THIS WRITES "HELLO!" THREE TIMES ON THE SAME LINE

turn to chapter Incantations
write "Hello! " under hello
sign chapter with ""
sign acknowledgements page with ""
publish spellbook
publish spellbook
publish spellbook
```

# The Drawer

Being a witch, you have a magical desk with a magical bottomless drawer, and it can hold a near infinite number of chapter pages.

This is your second option when tearing out a chapter: If you want to save the contents of that chapter for later, you can put it in the drawer.

```
tear out chapter and put it in the drawer
```

You can then retrieve that chapter from the drawer later and replace whatever chapter you're turned to with it.

```
take out a chapter from the drawer and put it back
```

However, keep two things in mind about the drawer:
- You have no way to identify what chapters those pages were originally in, so keep track! If you put a whole page in the wrong chapter, *all* of its values will be automagically converted to that chapter's type, and that's probably not good!
-  You don't have time to rummage through the pages in your infinite drawer to find a specific one, so you can only take out the last chapter you put in.

# The Cauldron

The second most important asset of a spellbook-writing witch besides their spellbook is their cauldron. This cauldron is fairly basic and can only accept chapters from your spellbook as its ingredients, but you can cast all sorts of spells on it to do whatever you'd like to those pages.

This is your third and final option when tearing out a chapter: tossing it in the cauldron.
```
tear out chapter and toss it in the cauldron
```

## Mixing modes

Something important to note is that this cauldron is exceptionally choosy, and will only allow itself to contain one chapter at a time. If it already contains a chapter and another is tossed in, the cauldron will attempt to merge them into one chapter, and will do so depending on what *mixing mode* it's in. The two chapters are merged entry-by-entry: The first entry of the chapter in the cauldron is mixed with the first entry of the chapter tossed in, and so on.

There are four different mixing modes:

### Entwinement
Values are combined. Numbers are added, and strings are concatenated onto each other.
```
turn to chapter Hexes
write 5 under num_1
tear out chapter and toss it in the cauldron
write 3 under num_2
tear out chapter and toss it in the cauldron
CAULDRON PAGE NOW CONTAINS 8
```

### Belittlement
Numbers are subtracted. If there is a string in the cauldron, mixing it with an integer will remove that many characters from it, and mixing it with another string will remove that substring from it.
```
turn to chapter Incantations
write "impossible" under string
tear out chapter and toss it in the cauldron
write "im" under string
tear out chapter and toss it in the cauldron
CAULDRON NOW CONTAINS "POSSIBLE"
```

### Reenactment
Numbers are multiplied. If there is a string in the cauldron, mixing it with an integer will repeat the string that many times.
```
turn to chapter Incantations
write "Hello" under string
tear out chapter and toss it in the cauldron
turn to chapter Hexes
write 4 under count
tear out chapter and toss it in the cauldron
CAULDRON NOW CONTAINS "HELLOHELLOHELLOHELLO"
```

### Apportionment
Numbers are divided.
```
turn to chapter Illusions
write 12.5 under num1
tear out chapter and toss it in the cauldron
write 3.1 under num2
tear out chapter and toss it in the cauldron
CAULDRON NOW CONTAINS 4.032
```

The current mixing mode can be changed using the appropriate spell, discussed a bit later on.

## Knocking over and collecting the cauldron's contents

The magical liquid in the cauldron is too concentrated and dangerous to touch, so there's no pulling the chapter out of there once it's in. Thankfully, there's an easy solution. Knock over the cauldron and grab the chapter off the floor.
```
knock over cauldron
pick up chapter off the floor and put it back
```

Note that just like when putting back a chapter from the drawer, the page's contents will be automagically converted to the current chapter's type if it doesn't match.

# Spells

What would a witch be without their spells? You can cast a menagerie of spells on your cauldron to perform various effects on the chapter that's in it, on the cauldron itself, or on any number of things.
```
cast Vacancy on the cauldron
```

## Magical charge

Every time you cast a spell, the cauldron's magical charge increases. By default, it increases by 1 every time, but this can change depending on what the current amplifier is. Various spells will both affect and take advantage of the current charge.

## List of spells

The spells you can use are as follows:

| Spell | Description | Charge | Can be Reverberated to |
| ------------- | -------- | ---------------------- | ------------------ |
| Vacancy | Does absolutely nothing. Can be useful as a target for Reverberate. | +1 | Yes |
| Coadjuvancy | Asks the user for input and writes the value into the chapter that's in the cauldron. If the type doesn't match, it will be automagically converted. If the conversion fails, the user will be asked for input again. | +1 | Yes |
| Antipodize | Inverts all values on the chapter currently in the cauldron. Boolean values are flipped, numbers are negated, and strings are reversed. The entry names of all the values will also be reversed. | +1 | Yes |
| Judgement | Requires a Presages chapter in the cauldron. If *any* of the values on the chapter are false, lines will be skipped equal to the current spell charge. | +1 | Yes |
| Reverberate | Jumps back to the first spell cast in the program, plus one spell per current level of charge. | Resets to 0 | No |
| Entwinement | Changes the cauldron's mixing mode to Entwinement. | +1 | Yes |
| Belittlement | Changes the cauldron's mixing mode to Belittlement. | +1 | Yes |
| Reenactment | Changes the cauldron's mixing mode to Reenactment. | +1 | Yes |
| Apportionment | Changes the cauldron's mixing mode to Apportionment. | +1 | Yes |
| Amplify | Increases the charge amplifier by 1, making spells increase the charge by +1 more. If Amplify is cast three times in a row with no other non-Vacancy spells in-between, the cauldron will overload catastrophically. | +0 | No |
| Diminish | Decreases the current charge by 1. | -1 | No |
| Quelch | Resets the charge to 0 and the amplifier to 1. | Resets to 0 | No |

# Memorization

