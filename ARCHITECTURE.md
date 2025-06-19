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
│   ├── upgradeable_stats.rs # アップグレード可能ステータス（新配置）
│   └── markers.rs         # マーカーコンポーネント
├── systems/             # ECSシステム実装
│   ├── initialization.rs   # 初期化システム
│   ├── combat_core.rs     # 戦闘コアシステム
│   ├── combat_end.rs      # 戦闘終了システム
│   └── upgrades.rs        # アップグレード・同期システム（新配置）
├── events/              # イベント定義
│   └── combat_events.rs   # 戦闘関連イベント
├── plugins/             # プラグインシステム ✅ 新規追加 (2025-06-19)
│   ├── combat.rs          # 戦闘関連プラグイン
│   ├── stats.rs           # ステータス管理プラグイン
│   ├── ui.rs              # UI管理プラグイン
│   └── player.rs          # プレイヤー管理プラグイン
├── ui/                  # UI専用モジュール
│   ├── setup.rs           # UI初期化
│   ├── combat_ui.rs       # 戦闘UI更新
│   └── tab_ui.rs          # タブシステム
└── main.rs             # メインアプリケーション（プラグイン登録のみ）
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

#### アップグレードシステム（components/upgradeable_stats.rs）
- `CurrentValue`, `BaseValue`, `UpgradeLevel` - ステータス値管理（Level → UpgradeLevel に名前変更）
- `UpgradeCost`, `UpgradeMultiplier`, `CostMultiplier` - アップグレード計算
- `UpgradeableStat` - アップグレード可能な統計の識別
- 型安全マーカー: `UpgradeableHp`, `UpgradeableAttack`, `UpgradeableDefense`, `UpgradeableSpeed`

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

#### アップグレードフロー（systems/upgrades.rs）
1. `upgradeable_stat_upgrade_system` - 経験値でステータス自動アップグレード
2. 個別同期システム - アップグレード後の戦闘ステータス同期
   - `hp_sync_system` - HP同期専用
   - `attack_sync_system` - 攻撃力同期専用  
   - `defense_sync_system` - 防御力同期専用
   - `speed_sync_system` - スピード同期専用

## 2. ECS設計違反とリファクタリング課題

### 2.1 長い関数（ECS原則違反）

#### ~~🔴 重要度: 高~~ ✅ **リファクタリング完了** (2025-06-19)
- ~~**`real_time_attack_system`**~~ ✅ **解決済み** (combat_core.rs:30-85, 55行)
  - ~~プレイヤーと敵の攻撃処理を一つの関数で処理~~
  - ~~責任の分離ができていない~~
  - **解決済み**: `player_attack_system`と`enemy_attack_system`に分離完了

- ~~**`sync_stats_system`**~~ ✅ **リファクタリング完了** (2025-06-19)
  - ~~複数のステータスタイプを文字列比較で処理~~
  - ~~拡張性が低く、型安全性に欠ける~~
  - **解決済み**: 型安全な個別同期システムに分離完了

#### 🟡 重要度: 中
- **`player_death_system`** (combat_end.rs:60-89, 29行)
  - 死亡処理、リバース、リスポーンを一つの関数で処理

### 2.2 複数コンポーネント書き込み（ECS原則違反）

#### 🔴 重要度: 高
- ~~**`sync_stats_system`**~~ ✅ **リファクタリング完了** (2025-06-19)
  - ~~5つの異なるコンポーネントに書き込み~~
  - ~~`CombatAttack`, `CombatDefense`, `CombatSpeed`, `MaxHp`, `CurrentHp`~~
  - ~~ECSの単一責任原則に違反~~
  - **解決済み**: 各コンポーネント専用の個別システムに分離

- **`combat_init_system`** - 複数のコンポーネントを一度に追加
  - 初期化処理としては妥当だが、分離可能

#### 🟡 重要度: 中
- **`rebirth_player_system`** - 複数のエンティティと多様なコンポーネントを作成

### 2.3 設計上の課題

#### アーキテクチャレベル
- ~~**UI処理の混在**: main.rsに200行以上のUI処理が混在~~ ✅ **解決済み** (UIPlugin分離完了)
- ~~**文字列ベース処理**: `sync_stats_system`でのstat名の文字列比較~~ ✅ **解決済み**
- ~~**大規模な初期化**: main.rsの複雑な初期化処理~~ ✅ **解決済み** (プラグインシステム導入)

#### コンポーネント設計
- ~~**重複するLevel**: `management_stats::Level`と`upgradeable_stat::Level`~~ ✅ **解決済み** (UpgradeLevel に変更)
- ~~**ステータス同期の複雑性**: BaseStats ↔ CombatStats間の同期~~ ✅ **解決済み**
- **命名の一貫性**: 一部のコンポーネント名が不明確

## 3. 今後のリファクタリング課題

### 3.1 システム分離【優先度: 高】

#### ~~戦闘システムの分離~~ ✅ **完了** (2025-06-19)
```rust
// ✅ 実装完了: ECS準拠の分離システム
player_attack_system()      // プレイヤー攻撃専用 (~18行)
enemy_attack_system()       // 敵攻撃専用 (~18行)
execute_attack_if_ready()   // 共有攻撃ロジック (~15行)
```

#### ~~ステータス同期システムの分離~~ ✅ **完了** (2025-06-19)
```rust
// ✅ 実装完了: 型安全な個別同期システム
hp_sync_system()      // HP同期専用 (~15行)
attack_sync_system()  // 攻撃力同期専用 (~10行)
defense_sync_system() // 防御力同期専用 (~10行)
speed_sync_system()   // スピード同期専用 (~10行)
```

### 3.2 コンポーネント再設計【優先度: 高】

#### ~~ステータス同期の型安全性向上~~ ✅ **完了** (2025-06-19)
```rust
// ✅ 実装完了: 型安全なマーカーコンポーネント
#[derive(Component)]
pub struct UpgradeableHp;      // HP用マーカー

#[derive(Component)]
pub struct UpgradeableAttack;  // 攻撃力用マーカー

#[derive(Component)]
pub struct UpgradeableDefense; // 防御力用マーカー

#[derive(Component)]
pub struct UpgradeableSpeed;   // スピード用マーカー

// 型安全なクエリ例:
Query<&CurrentValue, (With<UpgradeableHp>, Changed<CurrentValue>)>
```

#### ~~Level コンポーネントの統一~~ ✅ **完了** (2025-06-19)
```rust
// ✅ 重複解消完了
// Before: upgradeable_stat::Level と management_stats::Level の重複
// After: upgradeable_stats::UpgradeLevel に変更、重複解消
```

### ~~3.3 UIシステム分離【優先度: 中】~~ ✅ **完了** (2025-06-19)

#### ~~UI専用モジュール作成~~ ✅ **実装完了**
```rust
// ✅ 実装済み: UI専用プラグインシステム
src/plugins/ui.rs        // UIプラグイン (setup_ui, update_ui_system, tab_button_system)
src/ui/                  // UI専用モジュール
├── mod.rs
├── setup.rs             // UI初期化
├── combat_ui.rs         // 戦闘UI更新
└── tab_ui.rs            // タブシステム
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

## 5. 完了したリファクタリング

### 5.0 プラグインシステム導入 (2025-06-19)

#### 🎯 **解決した問題**
- **main.rsの巨大化**: 55行の初期化処理 → 13行のプラグイン登録のみ
- **関心の分離**: 複数責任の混在 → 4つの専用プラグインに分離
- **保守性の向上**: システム追加時の影響範囲を最小化
- **可読性の向上**: 各プラグインが単一の関心事に集中

#### 🏗️ **実装したアーキテクチャ**

**プラグインシステム構造**:
```rust
// src/plugins/mod.rs - プラグイン統合管理
pub use combat::CombatPlugin;     // 戦闘システム統合
pub use stats::StatsPlugin;       // ステータス管理統合
pub use ui::UIPlugin;             // UI管理統合  
pub use player::PlayerPlugin;     // プレイヤー管理統合
```

**各プラグインの責任分離**:
```rust
// PlayerPlugin: ゲーム状態とプレイヤー初期化
.insert_resource(GameState { ... })
.add_systems(Startup, player_init_system)

// CombatPlugin: 戦闘イベントと戦闘システム
.add_event::<AttackEvent>() + 6つの戦闘イベント
.add_systems(Update, (combat_init_system, attack_systems, death_systems, ...))

// StatsPlugin: アップグレード・同期システム  
.add_systems(Update, (upgradeable_stat_upgrade_system, hp_sync_system, ...))

// UIPlugin: UI初期化・更新システム
.add_systems(Startup, setup_ui)
.add_systems(Update, (update_ui_system, tab_button_system))
```

**簡潔になったmain.rs**:
```rust
// Before: 55行の複雑な初期化
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState { ... })
        .add_event::<AttackEvent>()
        // ... 50行以上の初期化処理
        .run();
}

// After: 13行のプラグイン登録のみ
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            PlayerPlugin,
            CombatPlugin,
            StatsPlugin,
            UIPlugin,
        ))
        .run();
}
```

#### ✅ **結果と検証**
- **コンパイル成功**: デバッグ・リリースビルド共に成功
- **機能リグレッションなし**: 全システムが正常動作
- **関心の分離**: 各プラグインが明確な責任を持つ
- **拡張性向上**: 新機能はプラグインとして独立追加可能

#### 🔧 **技術的詳細**
- **Before**: main.rs 55行（イベント7個 + システム15個 + リソース1個）
- **After**: main.rs 13行 + 4つの専用プラグイン
- **保守性**: 新システム追加時は該当プラグインのみ編集
- **テスタビリティ**: 各プラグインを独立してテスト可能

### 5.1 real_time_attack_system リファクタリング (2025-06-19)

#### 🎯 **解決した問題**
- **ECS原則違反の解消**: 55行の巨大システム → 2つの専用システム(各18行) + 共有ヘルパー(15行)
- **単一責任原則の実現**: プレイヤー・敵攻撃処理の混在 → 責任分離完了
- **コード重複の排除**: 重複ロジック → 共有ヘルパー関数で統一
- **拡張性の向上**: プレイヤー/敵固有機能の追加が容易

#### 🏗️ **実装したアーキテクチャ**

**分離システム構造**:
```rust
player_attack_system()      // プレイヤー攻撃専用 (~18行)
enemy_attack_system()       // 敵攻撃専用 (~18行) 
execute_attack_if_ready()   // 共通攻撃ロジック (~15行)
```

**ECS準拠の設計**:
```rust
// Before: 1つの巨大システム
pub fn real_time_attack_system(...) { /* 55行 */ }

// After: 責任分離 + 共有ヘルパー
pub fn player_attack_system(...) { /* 18行 */ }
pub fn enemy_attack_system(...) { /* 18行 */ }
fn execute_attack_if_ready(...) { /* 15行 */ }
```

**システム登録の更新**:
```rust
// main.rs:32
// Before: real_time_attack_system,
// After: (player_attack_system, enemy_attack_system),
```

#### ✅ **結果と検証**
- **コンパイル成功**: デバッグ・リリースビルド共に成功
- **全テスト通過**: 30個のテストケースすべて成功
- **機能リグレッションなし**: プレイヤー・敵攻撃システムが正常動作
- **ECS準拠**: 各システムが単一責任を持ち、適切に分離

#### 🔧 **技術的詳細**
- **Before**: プレイヤー攻撃(43-62行) + 敵攻撃(64-84行) = 55行システム
- **After**: 18行×2システム + 15行共有ヘルパー = 責任分離完了
- **重複排除**: 攻撃ロジックを`execute_attack_if_ready()`で統一
- **拡張性**: プレイヤー/敵固有のロジック追加が容易

### 5.2 sync_stats_system リファクタリング (2025-06-19)

#### 🎯 **解決した問題**
- **ECS原則違反の解消**: 40行の巨大システム → 4つの専用システム(各10-15行)
- **型安全性の向上**: 文字列ベースの判定 → コンパイル時型チェック
- **単一責任原則の実現**: 5コンポーネント同時変更 → 各システム1コンポーネント
- **保守性の向上**: 新しいステータス追加時の影響範囲を最小化

#### 🏗️ **実装したアーキテクチャ**

**型安全なマーカーコンポーネント**:
```rust
#[derive(Component)] pub struct UpgradeableHp;
#[derive(Component)] pub struct UpgradeableAttack;
#[derive(Component)] pub struct UpgradeableDefense;  
#[derive(Component)] pub struct UpgradeableSpeed;
```

**個別同期システム**:
```rust
pub fn hp_sync_system(...)      // MaxHp, CurrentHp の同期
pub fn attack_sync_system(...)  // CombatAttack の同期
pub fn defense_sync_system(...) // CombatDefense の同期
pub fn speed_sync_system(...)   // CombatSpeed の同期
```

**型安全なBundle**:
```rust
UpgradeableHpBundle::new(base_value, cost, multiplier, cost_multiplier)
UpgradeableAttackBundle::new(base_value, cost, multiplier, cost_multiplier)
// ... 他も同様
```

#### ✅ **結果と検証**
- **コンパイル成功**: デバッグ・リリースビルド共に成功
- **機能テスト**: 戦闘システム、経験値獲得、ステータス同期が正常動作
- **ECS準拠**: 各システムが単一責任を持ち、適切に分離
- **型安全**: ランタイムエラーの可能性を排除

#### 🔧 **技術的詳細**
- **Before**: `match stat.name.as_str() { "HP" => ... }`
- **After**: `Query<&CurrentValue, (With<UpgradeableHp>, Changed<CurrentValue>)>`
- **システム行数**: 40行 → 10-15行 x 4システム
- **型安全性**: ランタイム文字列比較 → コンパイル時型チェック

## 6. まとめ

### 現在の設計の良い点
- **イベント駆動**: 戦闘システムでイベントを活用した疎結合
- **コンポーネント分離**: 戦闘用・管理用ステータスの適切な分離
- **拡張可能性**: UpgradeableStatシステムの汎用設計

### 改善が必要な点
- ~~**システムの巨大化**: 複数責任を持つシステムの分割~~ ✅ **完了** (sync_stats_system + real_time_attack_system)
- ~~**型安全性**: 文字列ベース処理の型安全な実装への移行~~ ✅ **完了**
- ~~**UI分離**: メインロジックからのUI処理分離~~ ✅ **完了** (UIPlugin分離)
- ~~**main.rsの巨大化**: 初期化処理の分離~~ ✅ **完了** (プラグインシステム導入)

### 次期開発での重点事項
1. ~~**システム分割**: 50行超えシステムの細分化~~ ✅ **完了** (sync_stats_system + real_time_attack_system)
2. ~~**型安全性向上**: コンパイル時の型チェック強化~~ ✅ **sync_stats_system完了**
3. ~~**プラグインシステム導入**: 関心の分離とmain.rs簡略化~~ ✅ **完了** (4つのプラグインに分離)
4. **テスト整備**: 個別システムの単体テスト作成
5. **ドキュメント整備**: システム間の依存関係の文書化

### 残りの重要タスク
1. ~~**real_time_attack_system 分割**~~ ✅ **完了** (55行 → プレイヤー/敵攻撃システム分離)
2. ~~**UI分離**~~ ✅ **完了** (UIPlugin + UI専用モジュール分離)
3. ~~**プラグインシステム導入**~~ ✅ **完了** (main.rs 55行 → 13行に削減)
4. ~~**アップグレードシステム分離**~~ ✅ **完了** (2025-06-19) (components/systems分離)
5. ~~**Level コンポーネント統一**~~ ✅ **完了** (UpgradeLevel に変更、重複解消)
6. **テスト拡充** (特に新しく分離したシステムの単体テスト)

## 7. アップグレードシステム分離リファクタリング (2025-06-19)

### 7.1 解決した問題
- **アーキテクチャ不整合の解消**: `src/upgradeable_stat.rs` が根レベルに配置され、他のコンポーネント・システムと構成が異なっていた
- **関心の分離実現**: コンポーネントとシステムが同一ファイルに混在 → 適切なディレクトリに分離
- **命名衝突の解決**: `upgradeable_stat::Level` と `management_stats::Level` の重複 → `UpgradeLevel` に変更
- **ECS原則への準拠**: 他のモジュールと同じ構成パターンに統一

### 7.2 実装したアーキテクチャ

**移行前の構造**:
```rust
// Before: 根レベルに全て混在
src/upgradeable_stat.rs  // コンポーネント + システム + テスト (371行)
```

**移行後の構造**:
```rust
// After: 関心の分離完了
src/components/upgradeable_stats.rs  // コンポーネント定義・Bundle・ユーティリティ
src/systems/upgrades.rs              // アップグレード・同期システム
```

**コンポーネントの整理** (`components/upgradeable_stats.rs`):
```rust
// 基本コンポーネント
CurrentValue, BaseValue, UpgradeLevel  // Level → UpgradeLevel に変更
UpgradeCost, UpgradeMultiplier, CostMultiplier

// 型安全マーカー
UpgradeableHp, UpgradeableAttack, UpgradeableDefense, UpgradeableSpeed

// Bundle構成
UpgradeableStatBundle, UpgradeableHpBundle, UpgradeableAttackBundle, ...
```

**システムの整理** (`systems/upgrades.rs`):
```rust
// アップグレードシステム
upgradeable_stat_upgrade_system()     // 経験値による自動アップグレード
update_current_value_on_change()      // 値変更時の再計算

// 同期システム（combat_end.rs から移動）
hp_sync_system(), attack_sync_system()  // 各ステータス専用同期
defense_sync_system(), speed_sync_system()
```

### 7.3 結果と検証
- **コンパイル成功**: デバッグ・リリースビルド共に成功
- **全テスト通過**: 24個のテストケースすべて成功
- **機能リグレッションなし**: アップグレード・同期システムが正常動作
- **アーキテクチャ統一**: 他のモジュールと同じ構成パターンに準拠
- **命名衝突解消**: `management_stats::Level` と `UpgradeLevel` で明確に分離

### 7.4 技術的詳細
- **ファイル分離**: 371行の巨大ファイル → 適切な関心事ごとに分離
- **命名変更**: `upgradeable_stat::Level` → `upgradeable_stats::UpgradeLevel`
- **システム移動**: 同期システムを `combat_end.rs` から `upgrades.rs` に集約
- **インポート整理**: 古い `crate::upgradeable_stat::` 参照をすべて更新
- **後方互換性**: 既存のAPIを維持しつつ構造を改善

このアーキテクチャドキュメントは、コードベースの成長と共に定期的に更新し、ECS設計原則との整合性を維持することを推奨します。