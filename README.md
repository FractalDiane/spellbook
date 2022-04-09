# Introduction

Welcome, aspiring witch! If you're reading this, you must have a knack for magic, and you must be interested in the world of spellbook authoring and publishing. Consequently, it appears you're also interested in strange programming languages that make no sense from a practical standpoint. That's why **SPELLBOOK** is the language for you!

Spellbook is an esoteric programming language based around the magical shenanigans of a financially-challenged witch writing a book of spells. You have some useful, if exasperatingly limited, tools at your disposal, alongside a collection of spells to help you get the job done and publish your spellbook. [WRITE MORE HERE]

# Syntax

## Comments
A fundamental principle of magic is that it only works if you say the words of your spells as inaudibly as possible. Shouting will accomplish nothing but making a fool of yourself. Therefore, shout in all caps if you want to add comments.

```
HELLO THERE THIS IS A COMMENT!
```

## Chapters
Due to budget constraints, you have a *very* small spellbook to write in, with only four different single-page chapters. Not only that, but each chapter can only have three entries in it. The chapters available to you are as follows:

| Chapter Title | Contents |
| ------------- | -------- |
| Presages | Boolean values, `true` or `false`. |
| Hexes | Integer numbers. |
| Illusions | Floating-point numbers. |
| Incantations | Character strings. |

One nice thing about being a witch, though, is that you have magical extremely deep desk drawers that can hold an infinite number of pages. Plus, your spellbook is slightly magical and can regenerate new chapter pages once you tear them out.

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

## Tearing out and putting back chapters

You can only have three entries per chapter, and you're writing with a pen like a good author, so there's no erasing those entries once you write them. So, how do you get rid of them? Tear the entire chapter out with `tear out chapter`! The magical spellbook will generate a new blank one for you right away.

Of course, where are you going to put those chapters you tear out? You have three options.

If you're so dissatisfied with your work that you want it gone forever, you can throw it in the trash like the worthless garbage that it is.

```
tear out chapter and throw it in the trash
```

If you want to save the contents of that chapter for later, you can put it in your magical infinite desk drawer.

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