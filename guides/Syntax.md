# Syntax
This is the syntax guide for DotSunda

```
    '' = sunda content block
    || = sunda numbers pipe
    \ = escape character (so it dosent collide with DotSunda's characters)
```
These are just the legends, its just to clarify some syntax in the code
```
    "LITERAL" = literal character present at the content (case insensitive)
    <NAME> = node name
    ~ = joining nodes
    + = one or more nodes must be present
    () = parentheses (just to be clear for the matches, and not to be confused with LITERAL "(" and ")") 
```
If the legends above defined, you **must** follow the rules of the node/nodes unless negated or optionally made using
```
    ? = optional node
    * = optionally exist nodes
    ! = negated node (the variant doesn't accept the existence of the node)
```
Theres also clause legends
```
    OR = one of the nodes must match
    AND = every nodes applied on the `AND` clause must be matched
    IF(CONDITION) <ANY> = if the condition was true, then the node next tho the if clause exists
```
Common nodes:

```
    <Space> = " " OR <tab>
    <Newline> = (IF(WINDOWS) <CRLF>) OR (IF(LINUX OR MAC) <LF>)
    <Whitespace> = <Space> OR <Newline>
```
---

## Sunda Content

```
"'" ~ <Space>* ~ <Syllable>+ ~ <Space>* ~ "'"
```

For the content, it needs at least one or more syllables present at the sunda content block (`''`)

Inbetween the syllables, you can **optionally** add many `<Space>`s as you want. `<Newline>` is not supported at the moment

But the `<Space>`s inside Sunda Content doesn't mean that the transpiler will kept it onto the result content

### Syllables

Theres 6 variants for the syllables

#### Vocal
For the vocal character list, read more at [Character List file (#vocal-list)](./CharacterList.md#vocal-list).

The `<Vocal>` node consists of `"a" OR "i" OR "u" OR "e" OR "é" or <EAcuteAlias> OR "eu" OR "o"`

Since we know that most Indonesian (or maybe outside) uses querty keyboard, so we made an alias for é.. as `<EAcuteAlias>` (`^e`)

##### Vocal Only
```
    <Vocal>
```
For Example:
**'a'**, **'i'**
##### Vocal with Rarangken Suffix
```
    <Vocal> ~ <RarangkenSuffix>
```
For example: 'a**ng**', 'e**r**', 'a**ng**e**l**'

---

#### Consonant

For the consonant character list, read more at [Character List file (#consonant-list)](./CharacterList.md#consonant-list).

#### Ngalagena

Sundanese mostly uses `"a"` upon each `<Consonant>`, we'll call it `<Ngalagena>`. You can read more why on [Character List file (#ngalagena)](./CharacterList.md#ngalagena)

```
<Consonant> ~ "a"
```

Since below variants is very heavily depends on `<Ngalagena>` instead of `<Consonant>`, we'll make an explanation here to be clear

##### Ngalagena with Rarangken Suffix
The syntax is shown as this
```
<Consonant> ~ "a" ~ <RarangkenSuffix>
```
For the character list of `<RarangkenSuffix>`, you can read more at [Character List file (#rarangken-suffix)](./CharacterList.md#rarangken-suffix-list).

The rarangken suffix only appends `"h" OR "ng" OR "r"` at the end of the `<Ngalagena>`.
So for example: 'ka**ng**', 'ka**r**'

##### Ngalagena with Rarangken Replacer
The syntax is shown as this
```
<Consonant> ~ (!"a" AND <RarangkenReplacer>)
```
The logic is that, it dosent accept vocal `"a"` to be exists in this variant.

The `<RarangkenReplacer>` consists of 
```
(!"a" AND <Vocal>) OR <RarangkenRemover>
```
And the `<RarangkenRemover>`..
```
"-"
```
yeah, just a literal minus symbol.

The `<RarangkenReplacer>` replaces the vocal `"a"` to be either
- [\<Vocal\>](#vocal) **but not** `"a"` (`!"a" AND <Vocal>`), or
- `<RarangkenRemover>` node (`"-"`). Its a vocal killer that turns `<Ngalagena>` (`<Consonant> ~ "a"`) into [\<Consonant\>](./CharacterList.md#consonant-list) standalone

Example: 

- Normal `<RarangkenReplacer>`: 'k**e**', 'p**u**', 'p**o**c**i**', 'c**u**c**u**'
- `<RarangkenRemover>`: nga => '**ng**'

All combined would be:

'p**u**nya p**o**c**i**'

##### Ngalagena with Rarangken Infix
The syntax is shown as this
```
<Consonant> ~ <RarangkenSuffix> ~ "a"
```
For the character list of `<RarangkenSuffix>`, you can read more at [Character List file (#rarangken-suffix)](./CharacterList.md#rarangken-suffix-list).

The difference between [\<RarangkenReplacer\>](#ngalagena-with-rarangken-replacer) and the `<RarangkenInfix>` is that it dosent actually replaces the vocal `"a"`. It's just to add `"y" OR "l" OR "r"` inbetween `<Consonant>` and vocal `"a"` of the `<Ngalagena>`.


So for example: 'k**l**a', 'p**r**a'