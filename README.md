# jdoss_tide_level_converter「jtlc」
> 日本海洋データセンターの[毎時潮高データ](https://www.jodc.go.jp/vpage/tide_j.html)を変換するためのソフトウェア

* **必ずREADMEを最後まで読んでから、使用してください。**  

## Assets
|Asset|Note|
|---|---|
|[jtlc.exe](https://github.com/SL9-1994/jdoss_tide_level_converter/releases/latest/download/jtlc.exe)|x86_64-pc-windows-gnu|
|[jtlc](https://github.com/SL9-1994/jdoss_tide_level_converter/releases/latest/download/jtlc)|x86_64-unknown-linux-gnu|

## Usage

1. Pass the path of the directory containing the downloaded txt data to the -d option.

**Example**
下記のデータを例とします。
> コード：	      HD18  
> 験潮所名：	  博多  
> 緯度経度：	  33-37.117N 130-24.467E  
> 提供機関:       海上保安庁(HD)  
> データ提供期間： 1965 - 2021

1. データをダウンロードし、HD18.zipを展開
2. HD18が存在するディレクトリに実行ファイルを配置
3. jtlc.exeの-dオプションにHD18を渡す。
4. 変換が行われます。

```zsh
 $ jtlc.exe -d ./HD18
```

**Help**
```zsh
 $ jtlc --help

 CLI for converting hourly tide height data from Japan Oceanographic Data Center

 Usage: jtlc [OPTIONS]

 Options:
  -d, --dir-path <CONVERT_DIRECTORY_PATH>
          Pass the path of the directory where the txt file to be converted is stored
  -h, --help
          Print help
  -V, --version
          Print version
```

## Critical information

> 本ソフトウェアは素人が作成したものであり、重大なバグやエラーが含まれている可能性があります。  
> 私は本ソフトウェアを使用して発生した、いかなる問題にも責任を負いません。   
> 使用者の自己責任で使用してください。  

## Dependency
* clap        "4.5.4"

## License
This project is licensed under the [MIT License](/LICENSE).
