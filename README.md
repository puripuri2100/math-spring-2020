
2020年の春休みの数学の課題を解くために作成されたソフトウェアです。

RustとCargoをインストールした状態で

```
carugo run
```

とするとソフトウェアがビルドされ、同時に実行されます。


# 各関数の役割

- `func::permutation (n64:u64, r:u64) -> U2048` : nPrを計算します
- `func::combin (n64:u64, r:u64) -> U2048` : nCrを計算します
- `func::factorial (n: u64) -> U2048` : n!を計算します
- `func::number_of_combinations (all: u64, group: u64) -> U2048` : `all`人を`group`個のクラスに分けるときの組み合わせ総数を計算します
- `func::check_size (all: u64, group: u64, n: u64) -> String` : `all`人を`group`個のクラスに分けるとき、前のクラスの人が同じクラスに`n`人いるときの組み合わせ総数の概算を出し、`n * (10 ^ m)`の形の文字列で出力します

# 概算について

数字の上2桁の数字を取り小数にし（仮に`x.y`とする）、数字を文字列化したものの長さを取得することで桁数を出します（長さを仮に`l`とする）。
このとき、出てくる数は`x.y * (10 ^ (l - 1))`となっています。

---

[MIT license](https://github.com/puripuri2100/math-spring-2020/blob/master/LICENSE)のもと、公開されます.

Copyright (c) 2020 Naoki Kaneko (a.k.a. "puripuri2100")