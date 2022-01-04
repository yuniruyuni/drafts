// rustのslackで見かけた質問への回答
// パスが与えられる。このパスについて
// - ディレクトリであるなら、それ以下のファイル全体を列挙して対象とする
// - ファイルであるなら、そのファイルだけを対象として列挙する
// という挙動を実現したい。どうするか？

use std::path::PathBuf;
use std::fs::{read_dir, read_to_string, metadata};
use anyhow::Result;

fn main() -> Result<()> {
    // PathBufを構築。
    // Pathはsliceにあたるので
    // read_dir()の結果で生存期間の取り扱いが難しくなる。
    // 回避にはstd::borrow::Cow<'a, B>を使う手もあるのだけれど、
    // この場合はそもそもPathBufで実体を持っておけば十分。
    let path = PathBuf::from(r"./");
    // let path = PathBuf::from(r"./Cargo.toml"); // ファイル1つで試す場合はこちら

    // fileかdirectoryかを判定するためmetadata化
    let md = metadata(&path)?;

    // entriesは対象となるファイルパスのリストです。
    let entries = if md.is_dir() {
        // ディレクトリの場合は、read_dir()で
        // 該当のディレクトリ内のファイルをすべて列挙
        read_dir(&path)?
            .map(|e| Ok(e?.path()))
            .collect::<Result<Vec<_>>>()?
    } else {
        // 単一ファイルのパスであるなら、単にそれが対象ファイル。
        vec![ path ]
    };

    // 各ファイルを読み込んで改行により結合する
    let s = entries
        .iter()
        .flat_map(|p| read_to_string(p))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", s);
    Ok(())
}
