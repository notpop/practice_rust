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

}
