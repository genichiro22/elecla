# Rules

## Board

- Board
  - current_player
  - Turn
    - Step: [Up, Main]
      - Phase: [Begin, ..., End]
        - Chain
  - Player
    - Id
    - Index
    - Life
    - Mana
    - Domain: [Hand, Battlefield, Graveyard, Waiting, Library]
      - Rules[OpenToEveryone, OpenToMe, OpenToOpponents, Hidden]

- State
  - Turn
    - Step: [Up, Main]
      - Phase: [Begin, ..., End]
        - Chain


Example...
```yaml
Board: 
    Turn: 2
    Step: "Main"
    Phase: "Ongoing"
    Chain: 
    Players:
      - Id: 123456
        Index: 1
        Life: 20
        Mana: 3
        Hand: 
            - Card1
            - Card3
        Battlefield:
      - Id: 7890
        Index: 0
        Life: 20
        Mana: ...
```

## Turn Flow

ターンはドローステップ・メインステップからなる。
各ステップはステップ開始・ステップ中・ステップ終了の３段階のフェーズからなる。
ビギンフェイズにはルールで決められた行動を遂行し、その後すべてのチェーンを解決する。
オンゴーイングフェイズではルールで決められた行動を取れる。
メインの場合にはアクションを取れる。
ドローステップではステップ中にそのプレイヤーがドローする。
メインステップではステップ中にそのプレイヤーがアクションできる。

## Action

ActionはRequirementとEffectからなる。
- Action
    - Requirement: []
    - Effect: []

## Requirement

Requirement（ないこともある）によってActionの可否が決まる。
- Requirement: [[Cost, Discard, ]]

## Effect

Effectはドロー・ダメージ・領域移動などの複数個の組み合わせである。
Effect内に定義されたものを順番に処理していく。

Effect: [[DrawXCards, XDamageToOneTarget, MoveTargetCreatureToGraveyard]]

ゲームの最小事象。チェーン内のこれを解決するたびに状況起因処理をチェックする。

## Chain
Effectの連鎖。すべて自動で行われる。


## Card
- Card
    - Name
    - Cost
    - Type
    - Ability

