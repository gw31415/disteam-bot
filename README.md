# Disteam bot
[![Deploy to Fly](https://github.com/gw31415/disteam-bot/actions/workflows/fly.yml/badge.svg)](https://github.com/gw31415/disteam-bot/actions/workflows/fly.yml)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

- [INVITE LINK](https://discord.com/api/oauth2/authorize?client_id=1179069630728388759&permissions=268435472&scope=bot)

チームメンバー以外が見られないカテゴリを作成するための Discord-bot です。

## コマンド一覧

- `/mkteam`: チームを作成します。
    - `/mkteam repub`: チームメンバーがチャンネル作成権限を得ます。
    - `/mkteam feudal`: 代表者がチャンネル作成権限を持ちます。
    - `/mkteam monarchy`: チャンネル作成権限を与えません。

## デプロイ

### 環境変数

- `DISCORD_TOKEN` (Required)：Discord トークン

### 権限設定

以下の権限を有効化してください。

- `Manage Roles`
- `Manage Channels`

### Docker（[Fly.io](https://fly.io)など）を使用する。

Dockerでのデプロイを想定して、Docker用のFeature `docker`を用意しています。
詳しくは[Dockerfile](./Dockerfile)をお読みください。

## ライセンス

このプログラムは[AGPL-3.0](./LICENSE)の下でライセンスされています。

## Authors

gw31415 <git@amas.dev>
