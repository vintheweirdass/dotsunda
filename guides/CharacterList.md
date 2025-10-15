# Character List

Legend (not actually present in the parser or the actual manuscript):
```
   ~ = joining nodes
   "LITERAL" = literal character present at the content (case insensitive). This legend was only used on special cases in this file
   <NAME> = node name
```

## Vocal
### Vocal list

Heres the list of vocal characters
<details>
<summary>Click to expand</summary>

| Manuscript | Number |
|------------|---------|
| &#7043; | `a` |
| &#7044; | `i` |
| &#7045; | `u` |
| &#7048; | `e` |
| &#7046; | `é` |
| &#7049; | `eu` |
| &#7047; | `o` |
</details>

---

### Vocal additional info

`^e` is specially present at the syntax as an alias to `é`. Since mostly we use normal querty keyboards

## Number
### Number list

Heres the list of number characters
<details>
<summary>Click to expand</summary>

| Manuscript | Number |
|------------|---------|
| &#7088; | `0` |
| &#7089; | `1` |
| &#7090; | `2` |
| &#7091; | `3` |
| &#7092; | `4` |
| &#7093; | `5` |
| &#7094; | `6` |
| &#7095; | `7` |
| &#7096; | `8` |
| &#7097; | `9` |
</details>

---

### Design pattern for the DotSunda number syntax

On the actual manuscript, you need to use a pipe (`|`) to mark the number boundary. For example:

`100` = |&#7089;&#7088;&#7088;|

`67` = |&#7094;&#7095;| 

`365` = |&#7090;&#7094;&#7093;|

`7` = |&#7095;|

So in DotSunda, you **must** need to use the pipes (`||`) on normal latin numbers, if you want to convert them equivalent as the sunda manuscript. For example:

`|100|`, `|67|`, `|365|`, `|7|`

## Consonant


### Consonant List

Heres the list of consonant characters
<details>
<summary>Click to expand</summary>

| Manuscript | Spelling |
|------------|---------|
| &#7086; | `kh` |
| &#7050; | `k` |
| &#7054; | `c` |
| &#7058; | `t` |
| &#7061; | `p` |
| &#7066; | `y` |
| &#7069; | `w` |
| &#7052; | `g` |
| &#7055; | `j` |
| &#7059; | `d` |
| &#7064; | `b` |
| &#7067; | `r` |
| &#7087; | `sy` |
| &#7070; | `s` |
| &#7053; | `ng` |
| &#7057; | `ny` |
| &#7060; | `n` |
| &#7065; | `m` |
| &#7068; | `l` |
| &#7072; | `h` |
| &#7062; | `f` |
| &#7063; | `v` |
| &#7051; | `q` |
| &#7071; | `x` |
| &#7056; | `z` |
</details>

### Ngalagena
If you've searched about Sundanese manuscript guide, you may ecounter the `<Ngalagena>`. The difference is that Ngalagena does these

```
<Consonant> ~ "a"
```

it only adds the `<Vocal>` alongside the `<Consonant>`. Since on Sundanese, to spell those consonants you need a vocal `"a"` helper. Theres no way you can say `kh` or `p` without sounding like a cat growling or a fart

Also in DotSunda, the `"a"` is pretty much conditional depending on the situation. You can read more about it in the [Syntax Guide file (#ngalagena)](./Syntax.md#ngalagena)
