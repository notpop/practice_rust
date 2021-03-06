fn main() {
    println!("Hello, world!");
    // コメントアウトはphpとかと同様らしい

    // 型が結構あってコアなものは 数値型、文字列型(str)、タプル、配列
    // 標準ライブラリに用意されているものには Vec、Option、Resultなどがある

    // 符号あり整数はi 符号なし整数はu 浮動小数店はf
    // 変数の大きさは 8, 16, 32, 64, 128
    // 符号あり32bitはi32になる
    // fはf32, f64のみ

    // strと標準ライブラリに実装されているStringが存在する
    // strはスライスつってstartとそこからの長さであり、変更不可
    // Stringが可変可能なので普段のイメージのStringって感じ
    // str, StringはUTF-8の文字列データなのでお互いに変換可能
    // String => &strはメモリ圧迫ないけど逆はあるので注意

    let s1: String = String::from("Hello, World!");
    // String => &str
    let s2: &str = &s1;
    // &str => String
    let _s3: String = s2.to_string();

    // タプルは異なる型を納めることができる集合。　初めて聞く
    let mut t = (1, "2");
    // このmutは確か変数に対してつけるみたいなのどっかで読んだ
    //println!("{}", t.0.to_string());
    //println!("{}", t.1.to_string());
    t.0 = 2;
    t.1 = "3";
    //println!("{}", t.0.to_string());
    //println!("{}", t.1.to_string());
    // println!適当に触った感じ{}が変数を表してて引数の順番に対応してbindしてくれる。
    // 文字列に変換しないと怒られた上に"{}"みたいにダブルクォーテーションで囲む必要がある。

    // タプルに対して配列は単一な型の集合で配列のサイズは固定でコンパイル時に決まってないとだめ。
    // おそらく変数で型はi32でサイズ3かつ初期値右側
    let mut a: [i32; 3] = [0, 1, 2];
    // 固定でサイズ3だけどindex0に対して3という要素一つだけ初期値として格納されている状態
    let b: [i32; 3] = [0; 3];
    // はい、意味わからんくなった・・・
    // 多分bの解釈が間違ってる。
    a[1] = b[1];
    // 中身みてみればわかるやろ。
    //println!("{}", b[1]); 0が出力された
    a[2] = b[2];
    //println!("{}", b[2]); こいつ0が出力された
    // つまり[i32; 3]は長さの3であってるけど、右辺は[0; 3]これで0を三つなんじゃね？
    // println!("{}", b[0]); これも0やから予測は今のところ正しそう。
    // これもまたよくわからんけど出力結果は　[0, 0]
    // 配列参照は自動的にスライスとして扱われるらしいけど[start..end]
    // startからendじゃなくてstartとendしか表示されてなくね？
    //println!("{:?}", &a[1..3]);
    //println!("{:?}", &a[0..3]);
    // あ、[0, 0, 0]になった。
    // 初期値は[0, 1, 2]でindex 0,1,2で1と2を書き換えたから全ての要素が0だから[0,0,0]になるんかな？
    // でもendのあたいが3ってのは気になるけどね、長さじゃなくてendなら普通[0..2]じゃね？
    // よくわからん

    // 構造体
    // struct Person {
    //     name: String,
    //     age: u32,
    // }

    // let person = Person {
    //     name: String::from("NotPop"),
    //     age: 23,
    // };
    //　ちょこちょこ出てきてるこのfromって何やろか・・・
    // Stringクラスのstaticな関数なんかな？fromの引数からString型でデータ作るよ！的な

    // 列挙型
    // enum Event {
    //     Quit,
    //     KeyDown(u8),
    //     MouseDown { x: i32, y: i32},
    // }

    // let event1 = Event::Quit;
    // let event2 = Event::MouseDown { x: 10, y: 10 };
    // 正直phpとかjavaしかやってこなかったからこの辺りはいまいちわかっとらんけどそのうち使うやろからざっくり理解で。

    //  標準ライブラリの型

    // Option データが存在する場合と存在しない場合を表現できる列挙型
    // switchの可読性あげた強化版みたいなイメージ
    // pub enum Option<T> {
    //     // データいない時
    //     None,
    //     // データいる時
    //     Some(T),
    // }

    // Result 処理結果が成功かエラーかを表現できる列挙型
    // TとEは任意の型
    // pub enum Result<T, E> {
    //     // 成功の時
    //     Ok(T),
    //     // 失敗の時
    //     Err(Err),
    // }

    // pub enum Result<i32, String> {
    //     // 成功時は値
    //     Ok(i32),
    //     // 失敗はエラーメッセージ
    //     Err(String),
    // }

    // let result: Result<i32, String> = Ok(200);

    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("Err: {}", err),
    // }

    // let result: Result<i32, String> = Ok(200);
    // println!("code: {}", result.unwrap_or(-1));
    // let result: Result<i32, String> = Err("error".to_string());
    // println!("code: {}", result.unwrap_or(-1));
    // unwrap_orはOKならそのまま展開してだめなら引数を返す。

    // fn func(code: i32) -> Result<i32, String> {
    //     println!("code: {}", code);
    //     Ok(100)
    // }

    // and_then()はOkだった時のみ指定した関数を返すので
    // 下記の場合だと前者はOKだが後者はErrなので前者しか実行されない。
    // let result: Result<i32, String> = Ok(200);
    // let next_result = result.and_then(func);
    // let result: Result<i32, String> = Err("error".to_string());
    // let next_result = result.and_then(func);

    // 関数ないのエラーの処理を外だしできる?演算子らしい
    // エラーの処理無茶苦茶効率化できそうじゃね？
    // 感覚と想像だけどthrowできる感覚なんよね？
    // fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    //     let code = result?;
    //     println("code: {}", code);
    //     Ok(100);
    // }

    // Vecがベクタ型で内部の要素数を増減させられる。
    // 多分普段使ってる配列とかオブジェクトに一番近いのでは？
    // 初期値の設定
    // let v1 = vec![1, 2, 3, 4, 5];
    // 0で五つ埋める
    // let v2 = vec![0; 5];
    // Vecにはメソッドが定義されててpush(), pop()らしい
    // まんまArrayやんけ・・・・
    // 最後尾に要素の追加または削除ってこと
    // 普通にindexしていで中身抜き出すこともできる
    // let v = vec![1, 2, 3, 4, 5];
    // println!("{}", v[0]);

    // 要素が定義されてないところへのアクセスはエラー（パニック）を起こすみたい。
    // パニックは起きると処理の強制終了が行われる
    // getメソッドで取得すると定義されてない場合はNoneを返却する、上記の懸念は解消される
    // let v = vec![1, 2, 3, 4, 5];
    // for element in &v {
    //     println!("{}", element);
    // }
    // これは拡張for文って感じ

    // Box
    // IoTのカンファレンスで聞いたスタックの話やね。
    // スタックだとコンパイルする時にサイズわかってて固定サイズ出ないとだめなのに対して
    // Box使うとヒープ領域に保存されて要は必要な分だけ好きなタイミングでスペースを確保できて逆に
    // いらなくなったタイミングでメモリ開放できるみたいなやつやけど動作が複雑な分ロスが発生するよねってこと
    // Boxはヒープを確保してその場所のポインタをスタックに格納する

    // Boxでできるようになること
    // コンパイルの時に固定サイズでないとだめなところがサイズわからんくてもよくなる
    // 大きなサイズの型の値を渡す時にコピーではなくてポイント渡すだけ
    // 共通のトレイトを実装した様々な型を画一的にポインタで扱うこと

    // fn main() {
    //     let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    //     print(byte_array);
    // }

    // fn print(s: [u8]) {
    //     println!("{:?}", s);
    // }

    // doesn't have a size known at compile-timeって怒られた
    // note: all function arguments must have a statically known size

    // fn print(s: Box<[u8]>) {
    //     println!("{:?}", s);
    // }

    // fn main() {
    //     let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    //     print(Box::new(byte_array));

    //     let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
    //     print(Box::new(byte_array));
    // }

    // 実行しないとdead_codeになる
    // main();

    // 変数を束縛する場合にはコンパイラが型推測をしてくれて
    // こういう場合だと基本的にi32になる
    // 変更したい場合は明示的に定義しないとだめ
    // let immut_val = 10;
    // let mut mut_val = 20;

    // mut_val += immmut_val;

    // let v1: u64 = 10;
    // // 数字限定だが下記のような下記からでもよい
    // let v2 = 10u64;

    // constは絶対に変更不可
    // staticは変更可能にもすることができるが
    // グローバルだとどこからでも変更可能な値になるのでunsafeブロック内に入れる必要がある
    // 安全の保証ができないコードを実行する必要がある時それらのコードはunsafeブロックに入れないとだめらしい

    // constはコンパイラがビルドする時実際の値に置き換えられるのに対して
    // staticはバイナリファイルの特定のせくしょに配置される
    // もしコンパイル時には決まらないが実行時に決まる定数を定義したい場合はlazy_staticを使うと良い

    // 制御構文
    // let number = 1;
    // if 0 < number {
    //     println!("0 < number");
    // }
    // else if number < 0 {
    //     println!("number < 0");
    // }
    // else {
    //     println!("0 == number");
    // }

    // ifは式なのでif式を評価した値を変数に束縛することや、関数の引数にすることもできる
    // let number = 1;
    // let result = if 0 > number {
    //     number
    // }
    // else {
    //     -number
    // };

    // println!("{}", result);
    // ifが返す値の型はすべて揃っている必要がある

    // loop, for, whileが存在する
    // loopは内部の処理を繰り返し実行でき抜け出す時にはbreakを使用する
    // さらにloopは式なのでbreak戻り値をつけて抜けることが可能

    // let mut count = 0;

    // let result = loop {
    //     println!("{}", count);
    //     count += 1;
    //     if count == 10 {
    //         break count;
    //     }
    // };

    //println!("{}", result);

    // 文ではなく式ってところが自由度をかなりあげてるんやな

    // let mut count = 0;

    // while count < 10 {
    //     println!("count: {}", count);
    //     count += 1;
    // }

    // let count: i32;

    // for count in 0..10 {
    //     println!("count: {}", count);
    // }

    // let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // for element in &array {
    //     println!("element: {}", element);
    // }

    // loop, for, whileにはラベルをつけることができ、breakするときにそのラベル指定で抜けることができる
    // 'main: loop {
    //     println!("main loop start");
    //     'sub: loop {
    //         println!("sub loop start");

    //         'subと何も記載無しに変更すると永遠ループした
    //         break 'main;

    //         println!("sub loop end");
    //     }
    //     println!("main loop end");
    // }

    // match
    // let i: i32 = 5;
    // match i {
    //     1 => println!("1"),
    //     2 => println!("2"),
    //     3 => println!("3"),
    //     _ => println!("misc"),
    // }


    // enum Color {
    //     Red,
    //     Blue,
    //     Green,
    // }

    // 列挙子が足りないとエラー
    // let c = Color::Red;
    // match c {
    //     Color::Red => println!("Red"),
    //     Color::Blue => println!("Blue"),
    //     Color::Green => println!("Green"),
    // }
    // 網羅性の確認をし全ての列挙子に対する処理が存在するかチェックする

    // matchも式なので分岐処理を行った後の結果を変数に束縛することも可能



}
