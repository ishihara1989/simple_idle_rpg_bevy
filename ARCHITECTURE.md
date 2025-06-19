# Simple Idle RPG - 現在の設計とリファクタリング課題

## 1. 現在のアーキテクチャ概要

### 1.1 技術スタック
- **ECSフレームワーク**: Bevy ECS 0.16.1
- **数値計算**: `too_big_float` (BigFloat) - 大きな数値の計算に対応
- **ゲームタイプ**: リアルタイム戦闘のIdle RPG

### 1.2 プロジェクト構造
```
src/
├── components/          # ECSコンポーネント定義
│   ├── combat_stats.rs    # 戦闘用の一時的なステータス
│   ├── management_stats.rs # 管理用の永続的なステータス
│   └── markers.rs         # マーカーコンポーネント
├── systems/             # ECSシステム実装
│   ├── initialization.rs   # 初期化システム
│   ├── combat_core.rs     # 戦闘コアシステム
│   └── combat_end.rs      # 戦闘終了システム
├── events/              # イベント定義
│   └── combat_events.rs   # 戦闘関連イベント
├── upgradeable_stat.rs  # アップグレード可能ステータスシステム
└── main.rs             # メインアプリケーション + UI
```

### 1.3 ECSコンポーネント設計

#### 戦闘ステータス（combat_stats.rs）
- `CurrentHp`, `MaxHp` - HP管理
- `CombatAttack`, `CombatDefense`, `CombatSpeed` - 戦闘用ステータス
- `AttackCooldown` - リアルタイム戦闘用クールダウン
- `ExpReward`, `EnemyNumber` - 敵固有データ

#### 管理ステータス（management_stats.rs）
- `BaseAttack`, `BaseDefense`, `BaseSpeed`, `BaseHp` - 基本ステータス
- `Experience`, `Level`, `RebirthPoints` - 進行管理

#### マーカーコンポーネント（markers.rs）
- `Player`, `Enemy` - エンティティ識別
- `StatsText`, `CombatText` - UI要素識別
- `GameState` - ゲーム全体の状態管理（Resource）

#### アップグレードシステム（upgradeable_stat.rs）
- `CurrentValue`, `BaseValue`, `Level` - ステータス値管理
- `UpgradeCost`, `UpgradeMultiplier`, `CostMultiplier` - アップグレード計算
- `UpgradeableStat` - アップグレード可能な統計の識別

### 1.4 システムフロー

#### 初期化フロー
1. `player_init_system` - プレイヤーエンティティと基本ステータス作成
2. `combat_init_system` - 戦闘用ステータスの初期化と敵スポーン

#### リアルタイム戦闘フロー
1. `attack_cooldown_system` - スピードに基づくクールダウン減少
2. `real_time_attack_system` - 攻撃可能な場合のAttackEvent発火
3. `damage_application_system` - ダメージ適用とDeathEvent発火

#### 戦闘終了フロー
1. `death_detection_system` - 死亡検出とイベント分岐
2. `enemy_death_system` / `player_death_system` - 死亡処理
3. `exp_gain_system` - 経験値獲得
4. `next_enemy_spawn_system` - 次の敵スポーン

#### アップグレードフロー
1. `upgradeable_stat_upgrade_system` - 経験値でステータス自動アップグレード
2. `sync_stats_system` - アップグレード後の戦闘ステータス同期

## 2. ECS設計違反とリファクタリング課題

### 2.1 長い関数（ECS原則違反）

#### 🔴 重要度: 高
- **`real_time_attack_system`** (combat_core.rs:30-85, 55行)
  - プレイヤーと敵の攻撃処理を一つの関数で処理
  - 責任の分離ができていない

- **`sync_stats_system`** (combat_end.rs:118-158, 40行)
  - 複数のステータスタイプを文字列比較で処理
  - 拡張性が低く、型安全性に欠ける

#### 🟡 重要度: 中
- **`player_death_system`** (combat_end.rs:60-89, 29行)
  - 死亡処理、リバース、リスポーンを一つの関数で処理

### 2.2 複数コンポーネント書き込み（ECS原則違反）

#### 🔴 重要度: 高
- **`sync_stats_system`** - 5つの異なるコンポーネントに書き込み
  - `CombatAttack`, `CombatDefense`, `CombatSpeed`, `MaxHp`, `CurrentHp`
  - ECSの単一責任原則に違反

- **`combat_init_system`** - 複数のコンポーネントを一度に追加
  - 初期化処理としては妥当だが、分離可能

#### 🟡 重要度: 中
- **`rebirth_player_system`** - 複数のエンティティと多様なコンポーネントを作成

### 2.3 設計上の課題

#### アーキテクチャレベル
- **UI処理の混在**: main.rsに200行以上のUI処理が混在
- **文字列ベース処理**: `sync_stats_system`でのstat名の文字列比較
- **グローバル状態**: `GameState`リソースによる状態管理

#### コンポーネント設計
- **重複するLevel**: `management_stats::Level`と`upgradeable_stat::Level`
- **ステータス同期の複雑性**: BaseStats ↔ CombatStats間の同期
- **命名の一貫性**: 一部のコンポーネント名が不明確

## 3. 今後のリファクタリング課題

### 3.1 システム分離【優先度: 高】

#### 戦闘システムの分離
```rust
// 現在: real_time_attack_system (55行)
// 分離後:
player_attack_system()      // プレイヤー攻撃専用
enemy_attack_system()       // 敵攻撃専用
attack_coordination_system() // 攻撃順序調整
```

#### ステータス同期システムの分離
```rust
// 現在: sync_stats_system (40行)
// 分離後:
hp_sync_system()      // HP同期専用
attack_sync_system()  // 攻撃力同期専用
defense_sync_system() // 防御力同期専用
speed_sync_system()   // スピード同期専用
```

### 3.2 コンポーネント再設計【優先度: 高】

#### ステータス同期の型安全性向上
```rust
// 現在: 文字列ベース
match stat.name.as_str() { "HP" => ... }

// 提案: マーカートレイト
trait SyncableToHp {}
impl SyncableToHp for UpgradeableHp {}

// または: 専用コンポーネント
#[derive(Component)]
struct SyncToHp(Entity); // アップグレード可能エンティティを参照
```

#### Level コンポーネントの統一
```rust
// 重複解消
pub use upgradeable_stat::Level as UpgradeLevel;
pub use management_stats::Level as PlayerLevel;
```

### 3.3 UIシステム分離【優先度: 中】

#### UI専用モジュール作成
```rust
src/ui/
├── mod.rs
├── setup.rs     // UI初期化
├── combat_ui.rs // 戦闘UI更新
└── tab_ui.rs    // タブシステム
```

### 3.4 イベント駆動アーキテクチャ強化【優先度: 中】

#### より細かいイベント分割
```rust
// 攻撃関連
PlayerAttackEvent, EnemyAttackEvent // 攻撃者別イベント
DamageCalculatedEvent               // ダメージ計算結果
CriticalHitEvent                   // クリティカル専用

// ステータス関連  
StatUpgradeEvent<T>                // ジェネリック型でステータス特定
StatSyncRequestEvent               // 同期要求イベント
```

## 4. ECS設計の基本原則と指針

### 4.1 ECSアーキテクチャの核心概念

#### Single Responsibility Principle (単一責任原則)
- **1システム = 1つの明確な責任**
- システムは特定のコンポーネントの組み合わせのみを処理
- 複数の異なるコンポーネントタイプへの書き込みは避ける

#### Data-Driven Design (データ駆動設計)
- **コンポーネント = 純粋なデータ**
- ロジックはシステムに、データはコンポーネントに分離
- コンポーネント間の依存関係を最小化

#### Loose Coupling (疎結合)
- **イベントシステム活用**
- システム間の直接的な依存を避ける
- 状態変更は専用イベントを通じて通知

### 4.2 システム設計のベストプラクティス

#### システムサイズの目安
- **理想的な行数**: 20-30行以内
- **最大許容**: 50行（複雑な計算がある場合）
- **50行超過時**: 複数システムへの分割を検討

#### クエリ設計の原則
```rust
// ✅ Good: 単一の責任、明確なコンポーネント組み合わせ
fn hp_regen_system(
    mut query: Query<&mut CurrentHp, With<RegenComponent>>,
) {}

// ❌ Bad: 複数の責任、多様なコンポーネント
fn update_all_combat_stats(
    mut query: Query<(&mut CurrentHp, &mut CombatAttack, &mut CombatDefense)>,
) {}
```

#### イベント設計の原則
```rust
// ✅ Good: 特定の状況を表現
#[derive(Event)]
struct PlayerLevelUpEvent { new_level: u32 }

// ❌ Bad: 汎用的すぎる
#[derive(Event)]  
struct UpdateEvent { entity: Entity, data: String }
```

### 4.3 パフォーマンス考慮事項

#### クエリの最適化
- **Changed<T>フィルター**: 変更されたコンポーネントのみ処理
- **With/Without**: 不要なエンティティの除外
- **ParallelIterator**: 可能な場合は並列処理

#### システム順序の最適化
- **依存関係の明確化**: `.after()`, `.before()`の活用
- **SystemSet**: 関連システムのグループ化
- **条件付き実行**: `run_if()`による不要な実行の回避

### 4.4 テストとデバッグ

#### システムのテスタビリティ
```rust
// ✅ Good: テストしやすい小さなシステム
#[cfg(test)]
mod tests {
    #[test]
    fn test_hp_regen() {
        // 単一機能のテストが簡単
    }
}
```

#### デバッグフレンドリーな設計
- **明確な命名**: システム・コンポーネント名から機能が理解できる
- **適切なログ**: 重要な状態変更をログ出力
- **エラーハンドリング**: `unwrap()`の使用を避け、適切なエラー処理

## 5. まとめ

### 現在の設計の良い点
- **イベント駆動**: 戦闘システムでイベントを活用した疎結合
- **コンポーネント分離**: 戦闘用・管理用ステータスの適切な分離
- **拡張可能性**: UpgradeableStatシステムの汎用設計

### 改善が必要な点
- **システムの巨大化**: 複数責任を持つシステムの分割
- **型安全性**: 文字列ベース処理の型安全な実装への移行
- **UI分離**: メインロジックからのUI処理分離

### 次期開発での重点事項
1. **システム分割**: 50行超えシステムの細分化
2. **型安全性向上**: コンパイル時の型チェック強化
3. **テスト整備**: 個別システムの単体テスト作成
4. **ドキュメント整備**: システム間の依存関係の文書化

このアーキテクチャドキュメントは、コードベースの成長と共に定期的に更新し、ECS設計原則との整合性を維持することを推奨します。