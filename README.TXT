🙟 The Syllabic Keyboard Project 🙝


𐫱 Goal 𐫱

Develop an iOS application that lets Inuktitut speakers frictionlessly type in
syllabics.

Additional goals:
– Let users write Inuktitut in latin text, using the same system as syllabics.
– Let users convert syllabic ⇋ latin text quickly in the suggestions bar.
– Let users pick suggestions from the aforementioned bar.

Non goals:
– 🗙 Facilitate writing in English, French, or any other latin-based language.
– 🗙 Facilitate writing in Cree, Ojibwe, or any other syllabic-based language.
– 🗙 Facilitate use of emojis, other characters not found on standard keyboards.


𐫱 Motive 𐫱

Syllabics are widely used by Inuktitut speakers.

Nunavik            → ᓄᓇᕕᒃ
Natsilingmiutut    → ᓇᑦᓯᖕᒥᐅᑐᑦ
Inuktitut          → ᐃᓄᒃᑎᑐᑦ
Nunatsiavummiutut  → ᓄᓇᑦᓯᐊᕗᒻᒥᐅᑐᑦ

Latin scripts are also used for all Inuit languages, but tend to produce words
that are long and unwieldy. The effectiveness of syllabics can be debated, but
the fact remains that they are highly popular today.

Unfortunately, despite good Unicode support, there exist no high-tier syllabic
keyboards for mobile devices.

I believe that by taking inspiration from other keyboards, particularly the
Japanese keyboard on iOS, writing in Inuktitut can go from being a slow and
painful process to a fast and enjoyable one. Furthermore, the same keyboard
design could be easily adapted to write the latin script at a much faster rate
than using a standard English keyboard.

𐫱 𐫱 𐫱 𐫱 𐫱

I currently lack the hardware to develop an iOS app, and as such I have to
consider my current capabilities. Concepts for two seperate systems arise:

1. A Rust-based system that will serve as the core logic for my iOS application,
as well as a future Android application; and

2. An HTML/CSS based system, which will serve as my rapid protoype.


For the web application, I have apprehensions. Namely, the type of code, and the
way everything is fit together, is unlikely to resemble, in any form, any sort
of Swift UI code on iOS. That being said, I'm well acquainted with these
technologies, and I think it would be a great opportunity for rapid prototyping
and experimentation.

For the Rust-based system, things have been going very slowly. I believe this is
due to their not being too much logical code, and my fear that once I'm finished
the internals, I'll be unable to work on the external code any time soon, due to
lack of hardware. I would very much like to finish my Rust-based syllabic
converter, but I think it might be more worth it to focus on the web app for
now.

Perhaps I should try and create a greenfield project to isolate the behaviour of
individual keys. Namely, the pressing of the keys and the sliding motion. This
could be achieved fairly easily with JS.

a: The single key, unpressed.
b: The dialogue that appears around the key when held.

a)    b)
    → ABC
 E  → DEF
    → GHI

The behaviour should be fairly simple, and less prine to scope creep than the
syllabic converter, so it could be a good start. I already have an HTML/CSS
mockup of the keyboard, though it's not functional. Once this is done, I can
combine the two and see how things work.

Eventually, I could add the suggestions bar, etc.


𐫱 Constraints 𐫱

It's very difficult for me not to over-optimize.

For the parsing code, I must use a regex-based tokenization system for the
lexing, and the regex library must be the standard one.

The lexer will determine if a given word is likely Inuktut, using some very
basic heuristics that can be coded into regex tokenizer itself. Ie. email
adresses and Zip codes should be left as is, perhaps even all all-caps
abbreviations. There's a desire to make this functionality very clever, but a
user is going to be using the converter on small selections of text most of the
time, and will expect simple, predictable behaviour.

Only words comprised of only Latin Inuktuk characters can go on to the parsing
stage, and the parser should not fail to produce output at that point (as in,
the conversion function *can* fail, but given the filtered input, that should
not be expected).

The regex to determine if a word is Inuktut will not pass any additional
information to the conversion function, that function can do it's own parsing.


Today, I will write this lexer. (December 26th, 2023)



𐫱 Misc. ideas 𐫱

Backspace deletes one character, always. But backspace+swipe left means you
delete the previous word/suggestion that was placed.

Look at this project to allow for morpheme-level suggestions:
https://github.com/LowResourceLanguages/InuktitutComputing