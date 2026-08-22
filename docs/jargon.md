# jvot3 jargon

These are not necessarily defined 'in order'; you might have to do
some jumping around. Words specific to jvot3 have a boat emoji ⛵

## Spelling forms

### Standard spelling ⛵

Ordinary written Lojban: the only letters are
*abcdefg'ijklmnoprstuvxyz* -- note that this excludes *denpa bu*
(period) because it's equivalent to writing a space, and *slaka bu*
(comma) because it allows for illegal syllable sequences like
*\*au,e,n*.

### Annotated spelling ⛵

Standard spelling has a problem: *i* and *u* are overloaded. Depending
on position, they can be

- a **monophthong (Y)**: e.g. *citka*, *bangu*. The letters *e o a y*
  are also (always) monophthongs.
- an **onglide (G)** i.e. /j, w/ before a following vowel: e.g. *ia*,
  *ue*
- an **offglide** i.e. the second element of a falling diphthong:
  e.g. *ei*, *au*

Annotated spelling replaces onglides and offglides with unambiguous
characters:

| role        | *i* becomes | *u* becomes |
|-------------|-------------|-------------|
| onglide     | *q*         | *w*         |
| offglide    | *ĭ*         | *ŭ*         |
| monophthong | *i*         | *u*         |

Annotation is done by `prewords::mark_glides`.

## Phonological terms

**Sonorant (R)**: One of *lmnr*.

**Diphthong (F)**: One of *ai ei oi au*.

### Hard consonant (C) ⛵

Any consonant except *.y'y.* /h/ and *denpa bu* /ʔ/ and onglides,
concretely: *bcdfgjklmnprstvxz*. This distinction is made because
there are more limits on what non-hard consonants can be next to. Hard
consonants are all the ones that participate in clusters.

| ↓ before →?     |   hard consonants    | *.y'y.* |      *denpa bu*      |   onglides   |     offglides      | monophthongs |
|-----------------|:--------------------:|:-------:|:--------------------:|:------------:|:------------------:|:------------:|
| hard consonants |   if valid cluster   |   ❌    | at cmevla boundaries |      ❌      |         ❌         |      ✅      |
| *.y'y.*         |          ❌          |   ❌    |          ❌          |      ❌      |         ❌         |      ✅      |
| *denpa bu*      | at cmevla boundaries |   ❌    |          ✅          |      ✅      |         ❌         |      ✅      |
| onglides        |          ❌          |   ❌    |          ❌          |      ❌      |         ❌         |      ✅      |
| offglides       |          ✅          |   ✅    |          ✅          | if different |         ❌         |      ❌      |
| monophthongs    |          ✅          |   ✅    |          ✅          |      ✅      | if valid diphthong |      ❌      |

### Stressable vowel (V)

Any vowel except *y* (which can't be stressed): *aeiou*.

### Hard onset ⛵

A permissible syllable-initial cluster of hard consonants. Valid hard
onsets are

- a single hard consonant
- a valid initial pair
- three consonants C₁C₂C₃ where C₁C₂ and C₂C₃ are valid initials, C₁
  is a sibilant (*s z c j*), and C₃ is *l* or *r*.

Valid initial and medial pairs are defined by CLL and present in
`phonology.rs`.

### Consonantal syllable

A syllable consisting of a hard consonant followed by a (syllabic)
sonorant.

## Morphological terms

**Brivla**: A word with at least two hard consonants and two syllables
but that isn't a sequence of cmavo (see below). These are always
verbs.

**Gismu**: A root word, CCVCV or CVCCV with C limited to hard
consonants.

**Lujvo [also *-jvo*-]**: A compound word consisting of rafsi and
hyphens.

**Zi'evla [also -*zve*-]**: A freeword, just defined as any brivla
that isn't a gismu or lujvo.

**Rafsi**: CLL defines this as a potentially short form of a gismu (CF
CV'V CVC CCV CVCC CCVC CVCCV CCVCV). jvot3 extends it to also cover
zi'evla and short forms of them. We use **CLL rafsi** to refer only to
gismu rafsi.

**Hyphen**: One of *r n y 'y y' 'y'* when used to delimit rafsi. The
first two are used to prevent tosmabru, the last four are used at
zi'evla boundaries, and *y* is also used between invalid consonant
clusters.

**Cmevla**: A name word, anything that ends in a consonant. In spoken
Lojban these are mandatorily surrounded by *denpa bu*.

**Cmavo**: A particle, anything that isn't a brivla or a cmevla. These
look like (C|G)?(V|F)('V|'F)*.

### Unit ⛵

If you take a Lojban sentence and remove as many spaces as you can
(counting *denpa bu* as mandatory spaces) without needing to indicate
stress...

> *mijoilodotixnu cusimsa lokacemelbi ce*

the resulting things-that-look-like-words are units. This is basically
"split after every brivla and every mandatory pause". Luckily for us,
we don't actually need to know what the *brivla* is:

### Pre-brivla ⛵

The optional part of a unit that's the candidate brivla, after any
(zero or more) cmavo at the start. Once a pre-brivla is isolated, it's
one of

- a gismu
- a **pre-CLLjvo**, if it's a sequence of CLL rafsi + hyphens
- a **pre-zvejvo**, if it contains *y* and splitting on all *y*
  creates a list of pre-xunckujvo and/or pre-zi'evla (with the latter
  possibly missing their final vowel)
- a **pre-zi'evla**, otherwise

We use *pre-* to indicate that they still might be slinku'i or
sli'ykru (see below), or might have other internal problems
e.g. multiple consecutive *y*-nucleus syllables.

### Brivla evidence

Brivla evidence determines the first syllable of a unit that can't
belong to a cmavo.

|                      | generates evidence | passes evidence leftward |
|----------------------|:------------------:|:------------------------:|
| onset cluster        |         ✅         |            ❌            |
| coda                 |         ✅         |            ❌            |
| unstressable nucleus |        ✅*         |            ✅            |
| *h* onset            |         ❌         |            ✅            |

\* *y* nuclei only count as brivla evidence if not followed by another
y cmavo.

Onset and coda evidence may also be called **periphery evidence** (as
*periphery* = syllable - nucleus), and syllables with either of them
may be called **blocking** (as they don't pass evidence leftward).

## Brivla failure modes

### Tosmabru

When you have a candidate brivla that's invalid because some cmavo
fall off the start. For example *\*tosmabru* is an invalid brivla
because it falls apart into *to smabru*. If the candidate brivla is a
lujvo, this can be fixed by adding a hyphen, e.g. *tosymabru*, or
adjusting the leftmost rafsi, e.g. *tonsymabru*.

### Slinku'i

When you have a candidate brivla that's invalid because you can
prepend a CV cmavo and get a valid brivla. For example

- *\*slinku'i* is an invalid zi'evla because you can prepend *no* to
  get *nos-lin-ku'i*
- *\*klingonygu'e* is an invalid lujvo (of *klingona gugde*) because
  prepending *pa* gives *pak-lin-gon-y-gu'e*

### Sli'ykru

Like slinku'i except you have specifically a candidate lujvo and you
can prepend *any* cmavo so long as it doesn't contain *y*. For example
*\*sli'ykru* is an invalid lujvo because you can prepend *ua* to get
*uasli-'y-kru*, which is a valid lujvo because *uasli* is a zi'evla
and we want zi'evla to be able to appear anywhere in a lujvo.

The only sli'ykru case we know of is pre-brivla-initial *CCV'y*.
