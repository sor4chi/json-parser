# Json Parser（仮）

友人とRustのLT会をしたときに作ったものです。
構文解析を書いてみたいよね、JSONだったらBNFでもすぐ表せれれるレベルでやりやすいんじゃない？ということで作りました。

## `./json.bnf`のプレイグラウンド
注意: 仕様上改行タグやコメント付きJSON、ダブルクオート文字列には対応していません
[BNF Playground](https://bnfplayground.pauliankline.com/?bnf=%3Cjson%3E%20%3A%3A%3D%20%3Cobject%3E%20%7C%20%3Carray%3E%0A%3Cobject%3E%20%3A%3A%3D%20%22%7B%22%20%3Cmembers%3E%20%22%7D%22%0A%3Cmembers%3E%20%3A%3A%3D%20%3Cpair%3E%20%7C%20%3Cpair%3E%20%22%2C%22%20%3Cmembers%3E%0A%3Cpair%3E%20%3A%3A%3D%20%3Cstring%3E%20%22%3A%22%20%3Cvalue%3E%0A%3Carray%3E%20%3A%3A%3D%20%22%5B%22%20%3Celements%3E%20%22%5D%22%0A%3Celements%3E%20%3A%3A%3D%20%3Cvalue%3E%20%7C%20%3Cvalue%3E%20%22%2C%22%20%3Celements%3E%0A%3Cvalue%3E%20%3A%3A%3D%20%3Cstring%3E%20%7C%20%3Cnumber%3E%20%7C%20%3Cobject%3E%20%7C%20%3Carray%3E%20%7C%20%22true%22%20%7C%20%22false%22%20%7C%20%22null%22%0A%3Cstring%3E%20%3A%3A%3D%20%22%27%22%20%3Ccharacters%3E%20%22%27%22%0A%3Ccharacters%3E%20%3A%3A%3D%20%3Ccharacter%3E%20%7C%20%3Ccharacter%3E%20%3Ccharacters%3E%0A%3Ccharacter%3E%20%3A%3A%3D%20%5Ba-z%5D%0A%3Cnumber%3E%20%3A%3A%3D%20%3Cinteger%3E%20%7C%20%3Cinteger%3E%20%22.%22%20%3Cfraction%3E%20%7C%20%3Cinteger%3E%20%22.%22%20%3Cfraction%3E%20%3Cexponent%3E%20%7C%20%3Cinteger%3E%20%3Cexponent%3E%0A%3Cinteger%3E%20%3A%3A%3D%20%3Cdigit%3E%20%7C%20%3Cdigit%3E%20%3Cinteger%3E%0A%3Cdigit%3E%20%3A%3A%3D%20%5B0-9%5D%0A%3Cfraction%3E%20%3A%3A%3D%20%3Cdigit%3E%20%3Cfraction%3E%0A%3Cexponent%3E%20%3A%3A%3D%20%3Cexponent%3E%20%3Cexponent%3E%0A&name=)
