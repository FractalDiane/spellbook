# Introduction

Welcome, fellow witch! If you're reading this, you must be interested in the world of spellbook authoring and publishing. 

# Syntax

## Comments
Comments are accomplished by shouting. One of the first things you'll learn about magic is that you need to say the words of your spell as inaudibly as possible; shouting therefore does absolutely nothing.

Of course, shouting in text form is accomplished with appropriate use of the caps lock key.
```
HELLO THERE THIS IS A COMMENT!
```

## Chapters
Due to budget constraints, you have a *very* small spellbook to write in, with only five different single-page chapters. Not only that, but each chapter can only have three entries in it. The chapters available to you are as follows:

| Chapter Title | Contents |
| ------------- | -------- |
| Presages | Boolean values, `true` or `false`. |
| Hexes | Integer numbers. |
| Illusions | Floating-point numbers. |
| Incantations | Character strings. |
| Recipes | Functions. |

One nice thing about being a witch, though, is that you have magical extremely deep desk drawers that can hold an infinite number of pages. Plus, your spellbook is slightly magical and can regenerate new chapter pages once you tear them out.

## Turning to and writing in chapter pages

Before you write anything, you'll need to make sure you're turned to the right chapter, with the `turn to chapter` statement. Then you can use `write entry` to write something down in the page. Remember that each page can only have three entries!

```
turn to chapter Bogus
write entry my_float with value 2.5
write entry my_float_2 with value 5.0
write entry my_float_3 with value 7.5
shout WRITE ENTRY MY_FLOAT_4 WITH VALUE 10.0 -- ERROR! CAN ONLY PUT THREE ENTRIES ON A PAGE
```

If you try to write an entry of a certain type in the wrong page, the spellbook will automagically and implicitmagically convert it for you.

```
turn to chapter Boolean
write entry my_float with value 2.5
shout THIS WRITES A VALUE OF TRUE

turn to chapter Integer
write entry my_bool with value true
shout THIS WRITES A VALUE OF 1

turn to chapter Routine
shout WRITE ENTRY MY_INT WITH VALUE 1 -- ERROR! CAN'T CONVERT THAT
```

## Tearing out and putting back chapter pages

You can only have three entries per chapter, and you're writing with a pen like a good author, so there's no erasing those entries once you write them. So, how do you get rid of them? Tear the entire chapter out with `tear out chapter`! The magical spellbook will generate a new blank one for you right away.

Of course, where are you going to put those chapters you tear out? Well, if you're so dissatisfied with your work that you want it gone forever, you can throw it in the trash like the worthless garbage that it is.

```
tear out chapter Bogus and throw it in the trash
```

However, if you want to save the contents of that chapter for later, you can put it in your magical infinite desk drawer.

```
tear out chapter Bogus and put it in the drawer
```

You can then retrieve that chapter from the drawer with `take out page from the drawer`.

```
take out page from the drawer and put it in Bogus
```

However, keep two things in mind about the drawer:
- You have no way to identify what chapters those pages were originally in, so keep track! If you put a whole page in the wrong chapter, *all* of its values will be automagically converted to that chapter's type, and that's probably not good!
-  You don't have time to rummage through the pages in your infinite drawer to find a specific one, so you can only take out the last chapter you put in. (In the programming world we call this a stack)

## Publishing

Publishing with `publish spellbook` allows you to share your immaculately-written spellbook with the world. This means putting your spellbook into print, of course.

```
turn to page String
write entry hello as "Hello, world!"
publish spellbook to console
```

If the console isn't your idea of a target audience, you can also publish your spellbook to a file.

```
publish spellbook to "hello.txt"
```

However, something very important to keep in mind is that you can't just publish half of a 