# TicketServerについて

TicketServerは2022.12.3-12.10に行われた[ブロックチェーンハッカソン](https://todaiweb3.com/hackathon/)において、
グループHが作成したものの一部です。「イベント等におけるチケット販売をNFTで行うシステム」のうち、TicketServerは
イベント情報などのチケットに付随する情報を登録・取得する役割を担います。

# 本サーバーの動かし方

`cargo`がインストールされていない場合は、まず`cargo`をインストールします。  
サーバーを動作させるには、本プロジェクトのトップディレクトリ(TicketServerディレクトリの直下)で、次のコマンドを実行します。
```shell
cargo run
```

# APIについて

### イベント一覧の取得

`/events`に対してGETリクエストを送ると、イベント情報が入ったJSONデータが返されます。  
このサーバーをローカルで動かしている場合は、ブラウザで次のURLにアクセスすることで応答が確認できます。  
http://localhost:8080/events

### イベント情報の登録

`/new`に対してイベント情報を入れたJSONデータをPOSTすると、イベント情報を登録できます。  
例えばシェルからPOSTリクエストを送る場合は、次のようにします。
```shell
curl --request POST --header "Content-Type: application/json" --data '{"amount":500, "price":100, "event_name":"concert", "image":"https://hoge.com/fuga.jpg", "explanation":"hello"}' http://localhost:8080/new
```

### NFTの情報を表すJSONファイルの取得

NFTの詳細情報を表すJSONファイルは、このサーバーから取得することが出来ます。  
例えば、イベントIDが0, チケット番号が1であるようなNFTの情報を取得するには、以下のURLにブラウザからアクセスします。  
http://localhost:8080/nft/0/1.json
