# How to setup

## 1. Dockerfile を使う場合

```bash
$ docker build . -t prac_websocket_rust
$ docker run -it -d --name prac_websocket_rust -p 8080:8082 prac_websocket_rust
```

(docker build <Dockerfile が存在するディレクトリ> -t <Docker イメージ名> )

(-p xx:yy は外部からアクセスされるポート番号:コンテナ側のポート番号を指定)

## 2. docker-compose を使う場合

```
$ docker-compose up
```

## 3. websocket でテスト接続

```
$ wscat -c ws://localhost:8080
Connected (press CTRL+C to quit)
> World
< Hello World
```
