# Introduction

Welcome, aspiring witch! If you're reading this, you must have a knack for magic, and you must be interested in the world of spellbook authoring and publishing. Consequently, it appears you're also interested in strange programming languages that make no sense from a practical standpoint. That's why **SPELLBOOK** is the language for you!

Spellbook is an esoteric programming language based around the magical shenanigans of a financially-challenged witch writing a book of spells. You have some useful, if exasperatingly limited, tools at your disposal, alongside a collection of spells to help you get the job done and publish your spellbook.

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

Before you write anything, you'll need to make sure you're turned to the right chapter. Then you can write something down in the chapter, under whatever heading name you'd like. Remember that each chapter can only have three entries!

```
turn to chapter Illusions
write 2.5 under my_float
write 5.0 under my_float_2
write 7.5 under my_float_3
write 10.0 under my_float_4   CATASTROPHE! CAN ONLY PUT THREE ENTRIES ON A PAGE
```

Note that entries are always written into a page in order, filling the first empty slot. For example, if a new entry is written in a page with only its third entry filled, it will be written to the first entry.

If you try to write an entry of a certain type in the wrong page, the spellbook will automagically and implicitmagically convert it for you.

```
turn to chapter Presages
write 2.5 under my_float
THIS WRITES A VALUE OF TRUE

turn to chapter Hexes
write true under my_bool
THIS WRITES A VALUE OF ONE

turn to chapter Illusions
write "Hello" under my_string    CATASTROPHE! CAN'T CONVERT THAT
```

Entries can also be copied into the next available slot.
```
turn to chapter Hexes
write 5 under num
copy num under num2
```

## Tearing out chapters

You can only have three entries per chapter, and you're writing with a pen like a good author, so there's no erasing those entries once you write them. So, how do you get rid of them? Tear the entire chapter out! The magical spellbook will generate a new blank one for you right away.

Of course, where are you going to put those chapters you tear out? The first option is the simplest: If you're so dissatisfied with your work that you want it gone forever, you can throw it in the trash like the worthless garbage that it is. The other options will be discussed further on.

```
tear out chapter and throw it in the trash
```

## Publishing

Publishing with allows you to share your immaculately-written spellbook with the world. This means putting your spellbook into print, of course.

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
THIS PRINTS HELLO, WORLD! AND THEN FIFTY-TWO
```

Something else important to note is that published books need to be as perfectly wrapped up as possible, so by default your published spellbook will be terminated with a nice, fancy "THE END."

If you want to end off your spellbook with something else, sign the acknowledgements page whatever string you'd like.
```
STANDARD HELLO WORLD PROGRAM

turn to chapter Incantations
write "Hello, world!" under hello
sign acknowledgements page with "\n"
publish spellbook
```

You can also sign individual chapters with whatever closing you'd like. By default, they're signed with a newline.

```
THIS WRITES HELLO! THREE TIMES ON THE SAME LINE

turn to chapter Incantations
write "Hello! " under hello
sign chapter with ""
sign acknowledgements page with ""
publish spellbook
publish spellbook
publish spellbook
```

## Memorization

One value at a time of any type can be memorized and then written back down wherever you want.
```
turn to chapter Hexes
write 25 under num
memorize num
write from memory under num2

turn to chapter Incantations
write from memory under num3 AUTOMAGICALLY CONVERTS IT TO A STRING
```

However, memory can be unreliable, and writing an exceptionally long number or string from memory has a chance to produce incorrect results.

When writing a long number from memory, digits past a certain point have a chance to randomly become something else. When writing a long string from memory, words past a certain point have a chance to disappear or be replaced with "something."
```
turn to chapter Incantations
write "Hello this is a really long line of text that I'll probably forget some of" under string
memorize string
write from memory under string_copy
SOME WORDS ARE PROBABLY GONE OR REPLACED WITH SOMETHING
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

| Mixing Mode | Description |
| ------------- | -------- |
| Entwinement | Values are combined. Numbers are added, and strings are concatenated onto each other. |
| Belittlement | Numbers are subtracted. If there is a string in the cauldron, mixing it with an integer will remove that many characters from it, and mixing it with another string will remove that substring from it. |
| Reenactment | Numbers are multiplied. If there is a string in the cauldron, mixing it with an integer will repeat the string that many times. |
| Apportionment | Numbers are divided. |

The current mixing mode can be changed using the appropriate spell, discussed a bit later on.

## Collecting the cauldron's contents

The chapter that's in the cauldron can be removed at any time and put back in the book.
```
take out chapter from the cauldron and put it back
```

Note that just like when putting back a chapter from the drawer, the page's contents will be automagically converted to the current chapter's type if it doesn't match.

The cauldron can also be knocked over to get rid of its contents without putting them back.
```
knock over cauldron
```

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
| Juxtapose | Transforms the chapter in the cauldron into a Presages chapter, with its values set based on comparisons between the previous page's values. <li>Value 1 is true if the page's 2nd and 3rd values were not 0 or empty.</li> <li>Value 2 is true if the page's 1st value was greater than or equal to its 3rd value, either numerically or lexicographically.</li> <li>Value 3 is true if the page's 1st value was equal to its 2nd value. | +1 | Yes |
| Reverberate | Jumps back to the first spell cast in the program, plus one spell per current level of charge. | Resets to 0 | No |
| Entwinement | Changes the cauldron's mixing mode to Entwinement. | +1 | Yes |
| Belittlement | Changes the cauldron's mixing mode to Belittlement. | +1 | Yes |
| Reenactment | Changes the cauldron's mixing mode to Reenactment. | +1 | Yes |
| Apportionment | Changes the cauldron's mixing mode to Apportionment. | +1 | Yes |
| Amplify | Increases the charge amplifier by 1, making spells increase the charge by +1 more. If Amplify is cast three times in a row with no other non-Vacancy spells in-between, the cauldron will overload catastrophically. | +0 | No |
| Diminish | Decreases the current charge by 1. | -1 | No |
| Squelch | Resets the charge to 0 and the amplifier to 1. | Resets to 0 | No |
