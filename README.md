# Compression Tool Libray

Welcome to the Compression Tool Library. 

The library provides an api for lossless encoding and decoding of text files.

## 🚀 Quickstart

## 📚 Examples

### 1. `verify` ✅

Loads 100 books from the [Project Gutenberg library](https://www.gutenberg.org/), encodes them, decodes them, and verifies that the original texts have been recovered.

```bash
cargo run --example verify
```

```
...
📚 Succeeded: Contes et nouvelles                 214 KiB -> 122 KiB (56.9%)
📚 Succeeded: Citizen Bird                        586 KiB -> 354 KiB (60.4%)
📚 Succeeded: A West Country Pilgrimage           123 KiB ->  71 KiB (57.7%)
📚 Succeeded: Ten Reasons Proposed to His Adve... 238 KiB -> 141 KiB (59.2%)
📚 Succeeded: Pincher Martin, O.D.                694 KiB -> 413 KiB (59.5%)
📚 Succeeded: Rambling Recollections of Chelse... 108 KiB ->  62 KiB (57.5%)
📚 Succeeded: Dick Lionheart                      116 KiB ->  68 KiB (58.8%)
📚 Succeeded: Tales From Bohemia                  325 KiB -> 188 KiB (57.7%)
📚 Succeeded: Margret Howth, A Story of To-day    339 KiB -> 194 KiB (57.2%)
📚 Succeeded: Joy Ride                             35 KiB ->  21 KiB (60.6%)
📚 Succeeded: The Call of the Twentieth Centur...  89 KiB ->  52 KiB (57.7%)
📚 Succeeded: Speed the Plough                    138 KiB ->  85 KiB (61.4%)
📚 Succeeded: A Matter of Magnitude                32 KiB ->  20 KiB (61.5%)
📚 Succeeded: The Right of Way, Volume 1.         139 KiB ->  81 KiB (58.6%)
⠄ [00:00:03] [---------------------------------------------] 100/100 ( 0.0s)

🎉 Checks completed with no erros.
```

### 2. `table` 🪑

Loads a copy of the Project Gutenberg [Les Misérables, by Victor Hugo](https://www.gutenberg.org/files/135/135-0.txt), encodes it, and presents the occurences of each charecter as well as the calculated huffman code

```bash
cargo run --example table
```

```
+-----------+------------+---------------------------------+
| Character | Occurences | Huffman Code                    |
+-----------+------------+---------------------------------+
| ..        | ...        |                             ... |
| X         | 333        |                  11111101111111 |
| ..        | ...        |                             ... |
| t         | 223000     |                            1011 |
| ..        | ...        |                             ... |
+-----------+------------+---------------------------------+
```

### 3. `performance`

Loads a copy of the Project Gutenberg [Les Misérables, by Victor Hugo](https://www.gutenberg.org/files/135/135-0.txt), encodes it, then calculates the reduction in file size and the time taken.

```bash
cargo run --example statistics
```


```
+---------------+----------+
| Time Taken    | 49.70ms  |
+---------------+----------+
| Original size | 3.14 MiB |
+---------------+----------+
| Encoded size  | 1.77 MiB |
+---------------+----------+
| Ratio         | 56.3%    |
+---------------+----------+
```
