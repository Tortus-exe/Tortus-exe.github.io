# Purpose

You may be familiar with git, the time-bending, reality-warping tool for controlling versions of source code for your programs. I had always wondered, with life as complicated as source code, would it ever be possible to use these features of git to organize aspects of your life?

I was inspired by my journey through dental care. While going through dental care, I found myself going from one dentist to the next. Each dentist would say their own things, and it became my responsibility to make sense of it all. The conditions of my life, and the decisions trees that resulted of it, became too much to manage. Thus, I decided to create this tool.

# Overview

Today is January 1st, and for your new year's gift, you have just been contacted by a company asking for an interview. Congratulations! You want to do everything in your power to be accepted. Using this tool, you plan out the journey.

Everything starts with the present, which represents the root of the tree. After the present, you might imagine what happens during the interview process: the first interview on Jan 2nd, a technical interview on Jan 4th, and your first day on Jan 9th. Your timeline looks like this:

![timeline](?bubble_organizer_straight_timeline)

But what if the company does not believe you are a good fit after the first interview? Well, Jan 4th and Jan 9th won't happen, and you might have to pivot to a different company. Your timeline has branched! In one timeline you're successfully hired, and in another you're taking a different path. 

![timeline_branch](?bubble_organizer_branched_timeline)

This is the concept of the bubble organizer: it helps you manage all these branching timelines. To create a new timeline, run `bubble-organizer init -d "asked to be interviewed" -D 2026-01-01` to set the present. To display your tree, run `bubble-organizer print`. This will show something that looks like:

```
(1)
```

This timeline isn't very interesting: it only shows one event. Let's make some subsequent events! Running the following: 

```
bubble-organizer add-child --parent-id 1 -d "first interview" -D 2026-01-02
bubble-organizer add-child --parent-id 2 -d "technical interview" -D 2026-01-04
bubble-organizer add-child --parent-id 3 -d "first day!" -D 2026-01-09
```

allows us to create the following events in the timeline!

```
(1)
│
(2)
│
(3)
│
(4)
```

This shows the four events happening in order. But what if we don't get accepted? We can add a branch: `bubble-organizer add-child --parent-id 1 -d "hug mom" -D 2026-01-03 -c "did not get accepted into role"`. This will create a new entry in our timeline:

```
(1) 
│   
(2)
├───┐   
(3) (5)  
│
(4)
```

As you can see, there are two possibilities that event `2` (the first interview) can lead to. Either we will be accepted, or else we will hug mom. To see the description of an event, we just run `bubble-organizer get-description -i <ID>`.

```
2026-01-03: hug mom (if: did not get accepted into role)
```

# Future work

There are various directions such an organizer could take:
- integration with other OSes
- becoming an online service
- better visualization and UI
- integration with other self-planning apps

I personally find the possibilities very exciting! Please do not hesitate to contact me if you wish to ask questions. 