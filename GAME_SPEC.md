# Game Specification

This document describes the gameplay mechanics, balance systems, and core features of Simple Idle RPG.

## Game Overview

Simple Idle RPG is a real-time idle RPG where players battle enemies continuously, gain experience, and upgrade their stats to progress through increasingly difficult opponents. The game draws inspiration from incremental games like Cookie Clicker and Antimatter Dimensions.

**Core Philosophy**: Automation is key. When players return to the game, systems should be automated and progressing without manual intervention.

## 現在の実装状況 (Current Implementation Status)

### ✅ 完全実装済み (Fully Implemented)

#### コア戦闘システム (Core Combat System)
- **リアルタイム戦闘**: スピードに基づくクールダウンシステム
- **ダメージ計算**: attack - defense (最低1ダメージ)
- **死亡/復活**: 自動復活とボーナス付与
- **敵の自動生成**: 敵撃破後の次の敵出現

#### ステータス・アップグレードシステム (Stats & Upgrade System)
- **自動アップグレード**: 経験値を自動で最適な強化に使用
- **二重ステータス**: 管理用ステータス（永続）+ 戦闘用ステータス（一時）
- **プレイヤーステータス**: HP, Attack, Defense, Speed
- **指数的スケーリング**: コストと効果の指数的増加

#### 転生システム (Rebirth System)
- **転生ポイント**: 到達した敵レベルに基づいて獲得
- **永続ボーナス**: ステータス+10%/転生ポイント、コスト-5%/転生ポイント
- **ゲームリセット**: プログレス初期化 + 永続強化

#### UI・自動化 (UI & Automation)
- **タブインターフェース**: 戦闘タブ、転生タブ
- **リアルタイム更新**: 全ステータスの自動更新
- **完全自動化**: 戦闘、アップグレード、進行すべて自動
- **イベント駆動オートリトライ**: ゲームオーバー状態でのリアルタイム反応
- **統一された戦闘開始**: ボタン操作から自動リトライまで一貫したフロー

### 現在のゲームバランス (Current Game Balance)

#### 敵の強さスケーリング (Enemy Scaling)
```rust
// 実装済みの値
HP: 20.0 * 1.5^enemy_number
Attack: 3.0 * 1.3^enemy_number
Defense: 2.0 * 1.3^enemy_number
Speed: 0.8 * 1.1^enemy_number
EXP: 5.0 * 1.15^enemy_number
```

#### プレイヤー強化 (Player Upgrades)
```rust
// 実装済みの値
ステータス増加: 1.15倍/レベル
コスト増加: 1.3倍/レベル
基本コスト: 10 EXP
```

#### 転生ボーナス (Rebirth Bonuses)
```rust
// 実装済みの値
ステータスボーナス: (転生ポイント * 0.1 + 1.0)倍
コスト削減: 1.0 / (転生ポイント * 0.05 + 1.0)倍
```

## 計画中の機能拡張 (Planned Features)

### 🚧 短期実装予定 (Short-term Planned)

#### 基礎システム改善
- **セーブ/ロードシステム**: ゲーム進行の永続化
- **設定画面**: ゲームオプションとUI設定
- **転生UI改善**: 転生タブの詳細実装

#### 戦闘拡張
- **複数敵タイプ**: 特殊能力を持つ敵の追加
- **戦闘ログ**: 詳細な戦闘履歴表示
- **手動戦闘オプション**: 自動戦闘のON/OFF切り替え

### 📋 中期実装予定 (Medium-term Planned)

#### アンロック要素第1段階
**トリガー**: 初回死亡時
- **覚醒システム**: パッシブスキル解放
- **装備システム**: 基本的な数値強化装備
- **アクティブスキル**: 手動発動可能な特殊能力

#### ダンジョンシステム
- **連戦システム**: 複数敵との連続戦闘
- **チャレンジダンジョン**: 制限付き特殊戦闘
- **ボス戦**: 強力な単体敵との戦闘

### 📈 長期実装予定 (Long-term Planned)

#### 転生システム拡張
**トリガー**: 敵レベル100到達
- **転生パッシブスキル**: より強力な永続効果
- **戦闘外要素強化**: 経験値増加、自動化速度向上など
- **鍛冶システム**: 装備強化・作成システム

#### 昇天システム (Ascension)
**トリガー**: 転生システム習得後
- **ルーンシステム**: 装備強化要素
- **新しい進行軸**: 転生とは異なる成長システム
- **高次元的強化**: より大きなスケールの成長

#### 超越システム (Transcendence) 
**トリガー**: 昇天システム習得後
- **パートナーシステム**: 契約召喚獣・仲間システム
- **複合戦闘**: パートナーとの協力戦闘
- **メタ進行**: ゲーム全体のメタ要素

## ゲームバランス設計思想 (Game Balance Philosophy)

### 基本数値設計 (Base Numerical Design)
```
敵の強さ: レベルごとに1.3倍
報酬(EXP): それより低い割合、1.15倍程度
強化コスト: 段階ごとに1.3倍  
強化効果: 1.15倍程度
```

### バランス調整指針 (Balance Guidelines)
- **自然な行き詰まり**: 敵の強化 > 報酬の伸び により、アンロック要素なしでは必ず頭打ちになる
- **意味のある選択**: 各アンロック要素は異なる戦略を提供する
- **自動化の価値**: 手動操作よりも自動化が効率的になるよう設計
- **リセットの価値**: 転生・昇天・超越それぞれに明確なメリット

### インクリメンタル設計原則 (Incremental Design Principles)
1. **指数的成長**: すべての数値が指数的にスケールする
2. **レイヤー化**: 複数の成長システムが重なり合う
3. **自動化優先**: プレイヤーは戦略を考え、実行は自動化する  
4. **意味のあるリセット**: リセットにより新しい可能性が開ける

## 技術仕様 (Technical Specifications)

### アーキテクチャ (Architecture)
- **Engine**: Rust + Bevy ECS 0.16.1
- **Large Numbers**: too_big_float ライブラリ
- **Design Pattern**: Event-driven ECS with plugin system

### プラグイン構成 (Plugin Structure)
- **PlayerPlugin**: ゲーム状態初期化、プレイヤーセットアップ
- **CombatPlugin**: 戦闘イベント、リアルタイム戦闘システム、戦闘開始管理
- **StatsPlugin**: ステータス強化、同期システム
- **UIPlugin**: ユーザーインターフェース管理、更新、イベント発行
- **BalanceCheckPlugin**: ヘッドレステスト、自動化設定管理

### コンポーネント設計 (Component Design)
- **combat_stats.rs**: 一時的戦闘ステータス (CurrentHp, CombatAttack等)
- **management_stats.rs**: 永続進行ステータス (Experience, Level等)
- **upgradeable_stats.rs**: アップグレード可能ステータス定義

## 未確定要素・検討事項 (Undecided Elements)

### バランス調整 (Balance Tuning)
- [ ] より細かいスケーリング調整の必要性
- [ ] 各アンロック要素の解放タイミング
- [ ] 転生・昇天・超越の相対的価値バランス

### UI/UX改善 (UI/UX Improvements)
- [ ] より直感的なプログレス表示
- [ ] アンロック要素のプレビュー機能
- [ ] 戦略的選択肢の可視化

### 新システム詳細 (New System Details)
- [ ] パートナーシステムの具体的仕様
- [ ] 鍛冶システムの複雑さレベル
- [ ] チャレンジダンジョンの制限内容

## 開発優先度 (Development Priorities)

### Priority 1: 基盤強化
1. セーブ/ロードシステム
2. UI改善（アップグレードコスト表示等）
3. 基本設定システム

### Priority 2: コンテンツ拡張  
1. 複数敵タイプ
2. 覚醒システム
3. 基本装備システム

### Priority 3: 大型システム
1. ダンジョンシステム
2. 転生システム拡張
3. 昇天システム導入

---

## 最新の実装更新 (Latest Implementation Updates)

### 2025-06-20: イベント駆動オートリトライシステム
- **CombatStartEvent**: 全戦闘開始の統一イベント
- **即応オートリトライ**: ゲームオーバー中のボタン操作で即座に戦闘開始
- **責任分離**: UI・戦闘・死亡システム間の完全な疎結合
- **戦闘開始の一元化**: ダンジョンボタン・手動/自動リトライが同一フロー

---

**Last Updated**: 2025-06-20  
**Implementation Status**: Core systems and auto retry fully functional, event-driven architecture complete, expansion features planned