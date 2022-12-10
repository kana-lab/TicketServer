# TicketServerについて

TicketServerは2022.12.3-12.10に行われた[ブロックチェーンハッカソン](https://todaiweb3.com/hackathon/)において、
グループHが作成したものの一部です。暗号株式の売買システムのうち、TicketServerは企業に付随する情報を登録・取得する役割を担います。

# 本サーバーの動かし方

`cargo`がインストールされていない場合は、まず`cargo`をインストールします。  
サーバーを動作させるには、本プロジェクトのトップディレクトリ(TicketServerディレクトリの直下)で、次のコマンドを実行します。
```shell
cargo run
```

# APIについて

### イベント一覧の取得

`/list`に対してGETリクエストを送ると、企業情報が入ったJSONデータが返されます。  
このサーバーをローカルで動かしている場合は、ブラウザで次のURLにアクセスすることで応答が確認できます。  
http://localhost:8080/list

### イベント情報の登録

`/new`に対してイベント情報を入れたJSONデータをPOSTすると、企業情報を登録できます。  
例えばシェルからPOSTリクエストを送る場合は、次のようにします。
```shell
curl --request POST --header "Content-Type: application/json" --data '{"event_name":"crypto stock", "image":"https://hoge.com/fuga.jpg", "explanation":"hello", "address":"0x0000.."}' http://localhost:8080/new
```
